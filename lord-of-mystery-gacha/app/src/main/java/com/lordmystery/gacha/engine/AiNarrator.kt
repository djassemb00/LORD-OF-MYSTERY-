package com.lordmystery.gacha.engine

import android.os.Handler
import android.os.Looper
import org.json.JSONArray
import org.json.JSONObject
import java.net.HttpURLConnection
import java.net.URL
import java.net.URLEncoder
import kotlin.concurrent.thread

/**
 * AI narrator powered by FREE AI models.
 *
 * Providers:
 * - "local"       : offline template narrator (no internet needed)
 * - "pollinations": Pollinations.ai — completely free, NO API key needed
 * - "openrouter"  : OpenRouter free models (free key from openrouter.ai)
 * - "groq"        : Groq free tier (free key from console.groq.com)
 * - "gemini"      : Google Gemini free tier (free key from aistudio.google.com)
 *
 * If the network call fails for any reason, it silently falls back to the
 * local template narrator so the game never breaks.
 */
object AiNarrator {

    interface Callback {
        fun onResult(text: String, fromAi: Boolean)
        fun onError(message: String)
    }

    const val SYSTEM_PROMPT =
        "أنت راوي لعبة نصية مستوحاة من رواية «لورد الغامض» (Lord of the Mysteries). " +
        "اكتب بالعربية الفصحى بأسلوب غامض ومشوق. العالم مليء بالغموض والمسارات (الأحمق، الباب، الرائي، الشمس، الطاغية...) " +
        "والتسلسلات من 9 إلى 0، والملائكة، والكنائس، ونادي التاروت. صف ما يحدث باختصار (3-5 جمل) دون خيارات جديدة."

    fun narrate(
        save: SaveManager,
        prompt: String,
        callback: Callback
    ) {
        val provider = save.aiProvider
        if (provider == "local") {
            callback.onResult(localNarrate(prompt), false)
            return
        }
        thread {
            try {
                val text = when (provider) {
                    "pollinations" -> callPollinations(prompt)
                    "openrouter" -> callOpenAiCompatible(
                        "https://openrouter.ai/api/v1/chat/completions",
                        save.aiModel.ifEmpty { "meta-llama/llama-3.3-70b-instruct:free" },
                        save.aiKey, prompt
                    )
                    "groq" -> callOpenAiCompatible(
                        "https://api.groq.com/openai/v1/chat/completions",
                        save.aiModel.ifEmpty { "llama-3.3-70b-versatile" },
                        save.aiKey, prompt
                    )
                    "gemini" -> callGemini(save.aiModel.ifEmpty { "gemini-2.0-flash" }, save.aiKey, prompt)
                    else -> localNarrate(prompt)
                }
                Handler(Looper.getMainLooper()).post {
                    callback.onResult(text, true)
                }
            } catch (e: Exception) {
                val fallback = localNarrate(prompt)
                Handler(Looper.getMainLooper()).post {
                    callback.onResult(fallback, false)
                }
            }
        }
    }

    fun testConnection(save: SaveManager, callback: Callback) {
        narrate(save, "قل مرحباً بجملة واحدة قصيرة.", callback)
    }

    // ---------- Providers ----------

    private fun callPollinations(prompt: String): String {
        val full = URLEncoder.encode("$SYSTEM_PROMPT\n\n$prompt", "UTF-8")
        val url = URL("https://text.pollinations.ai/$full")
        val conn = url.openConnection() as HttpURLConnection
        conn.requestMethod = "GET"
        conn.connectTimeout = 30000
        conn.readTimeout = 60000
        conn.setRequestProperty("User-Agent", "LordOfMysteryGacha/1.0")
        val code = conn.responseCode
        if (code != 200) throw RuntimeException("Pollinations HTTP $code")
        return conn.inputStream.bufferedReader().readText().trim()
    }

    private fun callOpenAiCompatible(endpoint: String, model: String, key: String, prompt: String): String {
        val conn = URL(endpoint).openConnection() as HttpURLConnection
        conn.requestMethod = "POST"
        conn.connectTimeout = 30000
        conn.readTimeout = 60000
        conn.setRequestProperty("Content-Type", "application/json")
        if (key.isNotEmpty()) conn.setRequestProperty("Authorization", "Bearer $key")
        val body = JSONObject()
        body.put("model", model)
        val messages = JSONArray()
        messages.put(JSONObject().put("role", "system").put("content", SYSTEM_PROMPT))
        messages.put(JSONObject().put("role", "user").put("content", prompt))
        body.put("messages", messages)
        body.put("max_tokens", 400)
        conn.outputStream.use { it.write(body.toString().toByteArray(Charsets.UTF_8)) }
        val code = conn.responseCode
        val text = if (code == 200) conn.inputStream.bufferedReader().readText()
        else conn.errorStream?.bufferedReader()?.readText() ?: ""
        if (code != 200) throw RuntimeException("HTTP $code: $text")
        val json = JSONObject(text)
        return json.getJSONArray("choices").getJSONObject(0)
            .getJSONObject("message").getString("content").trim()
    }

    private fun callGemini(model: String, key: String, prompt: String): String {
        val url = URL("https://generativelanguage.googleapis.com/v1beta/models/$model:generateContent?key=$key")
        val conn = url.openConnection() as HttpURLConnection
        conn.requestMethod = "POST"
        conn.connectTimeout = 30000
        conn.readTimeout = 60000
        conn.setRequestProperty("Content-Type", "application/json")
        val body = JSONObject()
        val contents = JSONArray()
        val parts = JSONArray()
        parts.put(JSONObject().put("text", "$SYSTEM_PROMPT\n\n$prompt"))
        contents.put(JSONObject().put("parts", parts))
        body.put("contents", contents)
        conn.outputStream.use { it.write(body.toString().toByteArray(Charsets.UTF_8)) }
        val code = conn.responseCode
        val text = if (code == 200) conn.inputStream.bufferedReader().readText()
        else conn.errorStream?.bufferedReader()?.readText() ?: ""
        if (code != 200) throw RuntimeException("Gemini HTTP $code: $text")
        val json = JSONObject(text)
        return json.getJSONArray("candidates").getJSONObject(0)
            .getJSONObject("content").getJSONArray("parts").getJSONObject(0)
            .getString("text").trim()
    }

    // ---------- Local fallback narrator ----------

    private val LOCAL_LINES = listOf(
        "الضباب الرمادي يتحرك حولك كأن العالم نفسه يراقبك. تشعر أن كل خطوة تخطوها تغيّر شيئاً في هذا العالم الغامض.",
        "في مكان ما بعيد، يبتسم الأحمق خلف حجاب الغموض. مصيرك يتشكل الآن، وكل اختيار تفعله يترك أثراً.",
        "الرياح تحمل همسات من الماضي. تسمع صوتاً يقول إن الطريق أمامك طويل، لكن من يملك الشجاعة يصل إلى النهاية.",
        "تشعر بقوة روحانية تسري في جسدك. العالم من حولك يتغير ببطء، وكل شيء يبدو ممكناً الآن.",
        "من بعيد، ترى نوراً خافتاً. إنه نور الأمل في عالم مليء بالظلام والأسرار. واصل التقدم، فالمغامرة لم تبدأ بعد."
    )

    private fun localNarrate(prompt: String): String {
        val idx = (prompt.hashCode() and Int.MAX_VALUE) % LOCAL_LINES.size
        return LOCAL_LINES[idx]
    }
}
