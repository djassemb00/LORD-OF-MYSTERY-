package com.lordmystery.gacha

import android.app.Activity
import android.app.AlertDialog
import android.graphics.Color
import android.graphics.Typeface
import android.os.Bundle
import android.text.method.ScrollingMovementMethod
import android.util.TypedValue
import android.view.Gravity
import android.view.View
import android.widget.*
import com.lordmystery.gacha.data.CardDatabase
import com.lordmystery.gacha.data.GameCard
import com.lordmystery.gacha.data.Rarity
import com.lordmystery.gacha.data.WorldDatabase
import com.lordmystery.gacha.engine.AiNarrator
import com.lordmystery.gacha.engine.BattleEngine
import com.lordmystery.gacha.engine.GachaEngine
import com.lordmystery.gacha.engine.SaveManager
import com.lordmystery.gacha.engine.WorldEngine

/**
 * Lord of the Mysteries: Gacha — main activity.
 * Single-activity app with programmatic views (no XML layouts needed).
 * Fan-made game. Lord of the Mysteries belongs to Cuttlefish That Loves Diving.
 */
class MainActivity : Activity() {

    private lateinit var save: SaveManager
    private lateinit var root: LinearLayout
    private lateinit var content: FrameLayout
    private lateinit var topBar: LinearLayout
    private lateinit var navBar: LinearLayout

    // Theme colors
    private val bg = Color.parseColor("#0D0A1F")
    private val panel = Color.parseColor("#1B1533")
    private val gold = Color.parseColor("#D4AF37")
    private val textMain = Color.parseColor("#F5F0E1")
    private val textDim = Color.parseColor("#B8B0D8")

    private var currentScreen = "gacha"
    private var collectionFilter: Rarity? = null
    private val selectedTeam = mutableListOf<String>()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        save = SaveManager(this)

        root = LinearLayout(this)
        root.orientation = LinearLayout.VERTICAL
        root.setBackgroundColor(bg)
        setContentView(root)

        topBar = LinearLayout(this)
        topBar.orientation = LinearLayout.HORIZONTAL
        topBar.setBackgroundColor(panel)
        topBar.setPadding(dp(12), dp(10), dp(12), dp(10))
        root.addView(topBar, LinearLayout.LayoutParams(
            LinearLayout.LayoutParams.MATCH_PARENT, LinearLayout.LayoutParams.WRAP_CONTENT))

        content = FrameLayout(this)
        root.addView(content, LinearLayout.LayoutParams(
            LinearLayout.LayoutParams.MATCH_PARENT, 0, 1f))

        navBar = LinearLayout(this)
        navBar.orientation = LinearLayout.HORIZONTAL
        navBar.setBackgroundColor(panel)
        root.addView(navBar, LinearLayout.LayoutParams(
            LinearLayout.LayoutParams.MATCH_PARENT, LinearLayout.LayoutParams.WRAP_CONTENT))

        buildNav()
        showScreen(currentScreen)
    }

    // ---------- Helpers ----------

    private fun dp(v: Int): Int =
        TypedValue.applyDimension(TypedValue.COMPLEX_UNIT_DIP, v.toFloat(), resources.displayMetrics).toInt()

    private fun tv(text: String, size: Float = 16f, color: Int = textMain, bold: Boolean = false): TextView {
        val t = TextView(this)
        t.text = text
        t.textSize = size
        t.setTextColor(color)
        if (bold) t.typeface = Typeface.DEFAULT_BOLD
        t.setPadding(dp(4), dp(4), dp(4), dp(4))
        return t
    }

    private fun btn(text: String, bgColor: Int = gold, txtColor: Int = Color.BLACK, onClick: () -> Unit): Button {
        val b = Button(this)
        b.text = text
        b.textSize = 15f
        b.setTextColor(txtColor)
        b.setBackgroundColor(bgColor)
        b.setPadding(dp(12), dp(10), dp(12), dp(10))
        b.setOnClickListener { onClick() }
        return b
    }

    private fun panelBox(): LinearLayout {
        val l = LinearLayout(this)
        l.orientation = LinearLayout.VERTICAL
        l.setBackgroundColor(panel)
        l.setPadding(dp(12), dp(12), dp(12), dp(12))
        return l
    }

    private fun scrollBox(child: View): ScrollView {
        val s = ScrollView(this)
        s.addView(child, FrameLayout.LayoutParams(
            FrameLayout.LayoutParams.MATCH_PARENT, FrameLayout.LayoutParams.WRAP_CONTENT))
        return s
    }

    private fun refreshTopBar() {
        topBar.removeAllViews()
        val title = tv("🃏 لورد الغامض: غاتشا", 18f, gold, true)
        title.layoutParams = LinearLayout.LayoutParams(0, LinearLayout.LayoutParams.WRAP_CONTENT, 1f)
        topBar.addView(title)
        topBar.addView(tv("💎 ${save.essence}", 15f))
        topBar.addView(tv("🪙 ${save.coins}", 15f))
        topBar.addView(tv("⭐ Lv.${save.level}", 15f))
    }

    // ---------- Navigation ----------

    private fun buildNav() {
        navBar.removeAllViews()
        val items = listOf(
            "gacha" to "🃏 استدعاء",
            "world" to "🗺️ العالم",
            "collection" to "📚 البطاقات",
            "battle" to "⚔️ المعركة",
            "settings" to "⚙️ الإعدادات"
        )
        for ((id, label) in items) {
            val b = Button(this)
            b.text = label
            b.textSize = 12f
            b.setBackgroundColor(if (id == currentScreen) gold else panel)
            b.setTextColor(if (id == currentScreen) Color.BLACK else textDim)
            b.setOnClickListener {
                currentScreen = id
                buildNav()
                showScreen(id)
            }
            b.layoutParams = LinearLayout.LayoutParams(0, LinearLayout.LayoutParams.WRAP_CONTENT, 1f)
            navBar.addView(b)
        }
    }

    private fun showScreen(id: String) {
        content.removeAllViews()
        refreshTopBar()
        val view: View = when (id) {
            "gacha" -> gachaScreen()
            "world" -> worldScreen()
            "collection" -> collectionScreen()
            "battle" -> battleScreen()
            else -> settingsScreen()
        }
        content.addView(view, FrameLayout.LayoutParams(
            FrameLayout.LayoutParams.MATCH_PARENT, FrameLayout.LayoutParams.MATCH_PARENT))
    }

    // ---------- Gacha Screen ----------

    private fun gachaScreen(): View {
        val col = LinearLayout(this)
        col.orientation = LinearLayout.VERTICAL
        col.setPadding(dp(12), dp(12), dp(12), dp(12))

        val banner = panelBox()
        banner.addView(tv("✨ بوابة الضباب الرمادي ✨", 22f, gold, true))
        banner.addView(tv("استدعِ شخصيات وكائنات من عالم لورد الغامض. كل استدعاء قد يغيّر مصيرك.", 14f, textDim))
        banner.addView(tv("نسب الأسطوري: 12% | الملحمي: 25% | النادر: 35% | الشائع: 25% | الإلهي: 3% (+3% بعد 70 سحبة، مضمون عند 90)", 12f, textDim))
        col.addView(banner)

        col.addView(tv("سحبات حتى الإلهي المضمون: ${GachaEngine.HARD_PITY - save.pity}", 15f, gold, true))

        val row = LinearLayout(this)
        row.orientation = LinearLayout.HORIZONTAL
        val b1 = btn("سحبة واحدة\n💎 ${GachaEngine.COST_SINGLE}") { doPull(1) }
        b1.layoutParams = LinearLayout.LayoutParams(0, LinearLayout.LayoutParams.WRAP_CONTENT, 1f)
        val b10 = btn("10 سحبات\n💎 ${GachaEngine.COST_TEN}") { doPull(10) }
        b10.layoutParams = LinearLayout.LayoutParams(0, LinearLayout.LayoutParams.WRAP_CONTENT, 1f)
        row.addView(b1); row.addView(b10)
        col.addView(row)

        if (GachaEngine.dailyAvailable(save)) {
            val daily = btn("🎁 مكافأة يومية: 300 جوهر + 100 عملة") {
                if (GachaEngine.claimDaily(save)) {
                    Toast.makeText(this, "حصلت على المكافأة اليومية!", Toast.LENGTH_SHORT).show()
                    showScreen("gacha")
                }
            }
            col.addView(daily)
        }

        val results = LinearLayout(this)
        results.orientation = LinearLayout.VERTICAL
        results.id = View.generateViewId()
        col.addView(results)

        // Rebuild results area via tag lookup
        col.tag = results
        return scrollBox(col)
    }

    private fun doPull(count: Int) {
        val cost = if (count == 1) GachaEngine.COST_SINGLE else GachaEngine.COST_TEN
        if (save.essence < cost) {
            Toast.makeText(this, "جوهر غامض غير كافٍ! العب في العالم أو انتظر المكافأة اليومية.", Toast.LENGTH_LONG).show()
            return
        }
        save.essence -= cost
        val pulled = GachaEngine.pull(save, count)
        // Find results container (first child of scroll content)
        val scroll = content.getChildAt(0) as ScrollView
        val col = scroll.getChildAt(0) as LinearLayout
        val results = col.tag as LinearLayout
        results.removeAllViews()
        results.addView(tv("— نتائج الاستدعاء —", 16f, gold, true))
        for (card in pulled) {
            results.addView(cardRow(card))
        }
        refreshTopBar()
        Toast.makeText(this, "حصلت على ${pulled.size} بطاقة!", Toast.LENGTH_SHORT).show()
    }

    private fun cardRow(card: GameCard): View {
        val row = LinearLayout(this)
        row.orientation = LinearLayout.HORIZONTAL
        row.setBackgroundColor(panel)
        row.setPadding(dp(10), dp(8), dp(10), dp(8))
        val stripe = View(this)
        stripe.setBackgroundColor(card.rarity.color)
        stripe.layoutParams = LinearLayout.LayoutParams(dp(6), LinearLayout.LayoutParams.MATCH_PARENT)
        row.addView(stripe)
        val info = LinearLayout(this)
        info.orientation = LinearLayout.VERTICAL
        info.addView(tv("${card.emoji} ${card.nameAr}", 16f, textMain, true))
        info.addView(tv("${card.pathway} | تسلسل ${card.sequence} | ${card.rarity.labelAr} ${"★".repeat(card.rarity.stars)}", 13f, textDim))
        info.addView(tv("هجوم ${card.atk} | دفاع ${card.def} | صحة ${card.hp}", 12f, textDim))
        row.addView(info)
        row.setOnClickListener { showCardDetail(card) }
        return row
    }

    // ---------- Collection Screen ----------

    private fun collectionScreen(): View {
        val col = LinearLayout(this)
        col.orientation = LinearLayout.VERTICAL
        col.setPadding(dp(12), dp(12), dp(12), dp(12))

        val owned = save.getOwnedCards()
        col.addView(tv("📚 مجموعتك: ${owned.size} / ${CardDatabase.ALL.size} بطاقة", 18f, gold, true))

        // Filter chips
        val chipRow = LinearLayout(this)
        chipRow.orientation = LinearLayout.HORIZONTAL
        val filters = listOf<Rarity?>(null) + Rarity.values()
        for (f in filters) {
            val label = if (f == null) "الكل" else f.labelAr
            val chip = btn(label, if (collectionFilter == f) gold else panel, if (collectionFilter == f) Color.BLACK else textDim) {
                collectionFilter = f
                showScreen("collection")
            }
            chip.textSize = 12f
            chip.layoutParams = LinearLayout.LayoutParams(0, LinearLayout.LayoutParams.WRAP_CONTENT, 1f)
            chipRow.addView(chip)
        }
        col.addView(chipRow)

        val grid = GridLayout(this)
        grid.columnCount = 2
        grid.useDefaultMargins = true
        val list = if (collectionFilter == null) CardDatabase.ALL else CardDatabase.byRarity(collectionFilter!!)
        for (card in list) {
            val count = owned[card.id] ?: 0
            val cell = LinearLayout(this)
            cell.orientation = LinearLayout.VERTICAL
            cell.setBackgroundColor(if (count > 0) panel else Color.parseColor("#14101F"))
            cell.setPadding(dp(10), dp(10), dp(10), dp(10))
            val name = if (count > 0) "${card.emoji} ${card.nameAr}" else "❓ بطاقة مجهولة"
            cell.addView(tv(name, 14f, if (count > 0) textMain else textDim, true))
            cell.addView(tv("${card.pathway} | تسلسل ${card.sequence}", 12f, textDim))
            cell.addView(tv("${card.rarity.labelAr} ${"★".repeat(card.rarity.stars)}", 12f, card.rarity.color))
            if (count > 0) {
                cell.addView(tv("×$count | هجوم ${card.atk}", 12f, textDim))
                cell.setOnClickListener { showCardDetail(card) }
            }
            val params = GridLayout.LayoutParams()
            params.width = (resources.displayMetrics.widthPixels / 2) - dp(24)
            params.setMargins(dp(4), dp(4), dp(4), dp(4))
            cell.layoutParams = params
            grid.addView(cell)
        }
        col.addView(grid)
        return scrollBox(col)
    }

    private fun showCardDetail(card: GameCard) {
        val box = LinearLayout(this)
        box.orientation = LinearLayout.VERTICAL
        box.setPadding(dp(16), dp(16), dp(16), dp(16))
        box.addView(tv("${card.emoji} ${card.nameAr}", 20f, gold, true))
        box.addView(tv("${card.nameEn}", 13f, textDim))
        box.addView(tv("${card.pathway} (${card.pathwayEn}) | تسلسل ${card.sequence}", 14f))
        box.addView(tv("${card.rarity.labelAr} ${"★".repeat(card.rarity.stars)}", 14f, card.rarity.color))
        box.addView(tv("⚔️ هجوم: ${card.atk}   🛡️ دفاع: ${card.def}   ❤️ صحة: ${card.hp}   🔮 روحانية: ${card.spirit}", 14f))
        box.addView(tv("✨ مهارة: ${card.skillName} (تكلفة ${card.skillCost})", 14f, gold, true))
        box.addView(tv(card.skillDesc, 13f, textDim))
        box.addView(tv("📖 ${card.lore}", 13f, textDim))
        val aiText = tv("", 13f, textMain)
        box.addView(aiText)
        val aiBtn = btn("🤖 توليد قصة بالذكاء الاصطناعي") {
            aiText.text = "⏳ جاري التفكير..."
            AiNarrator.narrate(save, "اكتب قصة قصيرة (3 جمل) عن ${card.nameAr} من ${card.pathway}، تسلسل ${card.sequence}.", object : AiNarrator.Callback {
                override fun onResult(text: String, fromAi: Boolean) {
                    aiText.text = (if (fromAi) "🤖 " else "📝 ") + text
                }
                override fun onError(message: String) {
                    aiText.text = "تعذر الاتصال بالذكاء الاصطناعي."
                }
            })
        }
        box.addView(aiBtn)
        AlertDialog.Builder(this).setView(box).setPositiveButton("إغلاق", null).show()
    }

    // ---------- World Screen ----------

    private fun worldScreen(): View {
        val col = LinearLayout(this)
        col.orientation = LinearLayout.VERTICAL
        col.setPadding(dp(12), dp(12), dp(12), dp(12))

        val loc = WorldDatabase.location(save.locationId)
        val here = panelBox()
        here.addView(tv("${loc?.emoji ?: "📍"} أنت الآن في: ${loc?.nameAr ?: "؟"}", 20f, gold, true))
        here.addView(tv(loc?.desc ?: "", 14f, textDim))
        col.addView(here)

        // State panel
        val state = panelBox()
        state.addView(tv("🧠 العقل: ${save.sanity} | 🔮 الروحانية: ${save.spirituality}", 14f))
        state.addView(tv("🌙 فريق الليل: ${save.repNighthawks} | ⛈️ كنيسة العاصفة: ${save.repStorms}", 13f, textDim))
        state.addView(tv("🃏 نادي التاروت: ${save.repTarot} | 🌅 جماعة الفجر: ${save.repAurora} | ⛪ الكنيسة: ${save.repChurch}", 13f, textDim))
        if (save.getFlags().isNotEmpty()) {
            state.addView(tv("🚩 أحداث العالم: ${save.getFlags().joinToString(", ")}", 13f, gold))
        }
        col.addView(state)

        // Travel
        col.addView(tv("🧭 السفر إلى:", 16f, gold, true))
        if (loc != null) {
            for (connId in loc.connections) {
                val dest = WorldDatabase.location(connId) ?: continue
                val tb = btn("${dest.emoji} ${dest.nameAr} (خطر ${dest.danger}/10)") {
                    val msg = WorldEngine.travel(save, connId)
                    Toast.makeText(this, msg, Toast.LENGTH_LONG).show()
                    showScreen("world")
                }
                col.addView(tb)
            }
        }

        // Explore
        val explore = btn("🔍 استكشف المنطقة (لقاء عشوائي)") { showEncounter() }
        col.addView(explore)

        // AI narrate
        val aiBox = panelBox()
        val aiText = tv("🤖 الراوي الذكي: اضغط الزر لوصف ما يحدث حولك الآن.", 13f, textDim)
        aiBox.addView(aiText)
        val aiBtn = btn("🤖 اسأل الراوي الذكي") {
            aiText.text = "⏳ جاري الاتصال بالذكاء الاصطناعي..."
            val prompt = "صف مشهداً غامضاً يحدث الآن في ${loc?.nameAr}. ${WorldEngine.worldSummary(save)}"
            AiNarrator.narrate(save, prompt, object : AiNarrator.Callback {
                override fun onResult(text: String, fromAi: Boolean) {
                    aiText.text = (if (fromAi) "🤖 " else "📝 ") + text
                }
                override fun onError(message: String) {
                    aiText.text = "تعذر الاتصال. تحقق من الإنترنت أو مفتاح API في الإعدادات."
                }
            })
        }
        aiBox.addView(aiBtn)
        col.addView(aiBox)

        // Rest / Work
        val row = LinearLayout(this)
        row.orientation = LinearLayout.HORIZONTAL
        val rest = btn("😴 راحة (20 عملة)") {
            if (save.coins < 20) { Toast.makeText(this, "عملات غير كافية", Toast.LENGTH_SHORT).show(); return@btn }
            WorldEngine.rest(save)
            Toast.makeText(this, "استرحت واستعدت روحانيتك.", Toast.LENGTH_SHORT).show()
            showScreen("world")
        }
        rest.layoutParams = LinearLayout.LayoutParams(0, LinearLayout.LayoutParams.WRAP_CONTENT, 1f)
        val work = btn("💼 عمل (60 عملة)") {
            WorldEngine.work(save)
            Toast.makeText(this, "عملت وكسبت 60 عملة.", Toast.LENGTH_SHORT).show()
            showScreen("world")
        }
        work.layoutParams = LinearLayout.LayoutParams(0, LinearLayout.LayoutParams.WRAP_CONTENT, 1f)
        row.addView(rest); row.addView(work)
        col.addView(row)

        return scrollBox(col)
    }

    private fun showEncounter() {
        val enc = WorldEngine.randomEncounter(save)
        if (enc == null) {
            Toast.makeText(this, "لا توجد لقاءات هنا الآن. جرّب مكاناً آخر.", Toast.LENGTH_SHORT).show()
            return
        }
        val box = LinearLayout(this)
        box.orientation = LinearLayout.VERTICAL
        box.setPadding(dp(16), dp(16), dp(16), dp(16))
        box.addView(tv("⚡ ${enc.title}", 20f, gold, true))
        box.addView(tv(enc.text, 15f))
        val dialog = AlertDialog.Builder(this).setView(box).create()
        for ((i, choice) in enc.choices.withIndex()) {
            val cb = btn("${i + 1}. ${choice.text}", panel, textMain) {
                if (choice.battle) {
                    dialog.dismiss()
                    startBattleFromWorld()
                    return@btn
                }
                val result = WorldEngine.applyChoice(save, choice)
                dialog.dismiss()
                val resBox = LinearLayout(this)
                resBox.orientation = LinearLayout.VERTICAL
                resBox.setPadding(dp(16), dp(16), dp(16), dp(16))
                resBox.addView(tv("📜 النتيجة", 18f, gold, true))
                resBox.addView(tv(result, 15f))
                // AI reaction
                val aiText = tv("", 13f, textDim)
                resBox.addView(aiText)
                val resDialog = AlertDialog.Builder(this).setView(resBox).setPositiveButton("متابعة", null).create()
                resDialog.show()
                AiNarrator.narrate(save, "علق على هذا الاختيار: ${choice.text}. النتيجة: $result. ${WorldEngine.worldSummary(save)}", object : AiNarrator.Callback {
                    override fun onResult(text: String, fromAi: Boolean) {
                        aiText.text = (if (fromAi) "🤖 " else "📝 ") + text
                    }
                    override fun onError(message: String) { aiText.text = "" }
                })
                refreshTopBar()
            }
            box.addView(cb)
        }
        dialog.show()
    }

    private fun startBattleFromWorld() {
        val owned = save.getOwnedCards().keys.mapNotNull { CardDatabase.byId(it) }
        if (owned.size < 1) {
            Toast.makeText(this, "تحتاج بطاقة واحدة على الأقل للقتال! استدعِ أولاً.", Toast.LENGTH_LONG).show()
            return
        }
        currentScreen = "battle"
        buildNav()
        showScreen("battle")
        Toast.makeText(this, "اختر فريقك ثم ابدأ المعركة!", Toast.LENGTH_LONG).show()
    }

    // ---------- Battle Screen ----------

    private fun battleScreen(): View {
        val col = LinearLayout(this)
        col.orientation = LinearLayout.VERTICAL
        col.setPadding(dp(12), dp(12), dp(12), dp(12))

        val owned = save.getOwnedCards().keys.mapNotNull { CardDatabase.byId(it) }
        col.addView(tv("⚔️ اختر فريقك (حتى 3 بطاقات): ${selectedTeam.size}/3", 18f, gold, true))

        if (owned.isEmpty()) {
            col.addView(tv("لا تملك أي بطاقة بعد! اذهب إلى الاستدعاء أولاً.", 15f, textDim))
            return scrollBox(col)
        }

        for (card in owned) {
            val selected = selectedTeam.contains(card.id)
            val cb = btn("${if (selected) "✅" else "⬜"} ${card.emoji} ${card.nameAr} (${card.rarity.labelAr})", if (selected) gold else panel, if (selected) Color.BLACK else textMain) {
                if (selected) selectedTeam.remove(card.id)
                else if (selectedTeam.size < 3) selectedTeam.add(card.id)
                else Toast.makeText(this, "الفريق ممتلئ (3 كحد أقصى)", Toast.LENGTH_SHORT).show()
                showScreen("battle")
            }
            col.addView(cb)
        }

        val loc = WorldDatabase.location(save.locationId)
        val fight = btn("🔥 ابدأ المعركة في ${loc?.nameAr ?: ""} (خطر ${loc?.danger ?: 1}/10)") {
            if (selectedTeam.isEmpty()) {
                Toast.makeText(this, "اختر بطاقة واحدة على الأقل", Toast.LENGTH_SHORT).show()
                return@btn
            }
            val team = selectedTeam.mapNotNull { CardDatabase.byId(it) }
            val enemies = BattleEngine.generateEnemies(loc?.danger ?: 1, save.level)
            val result = BattleEngine.simulate(team, enemies)
            save.coins += result.coins
            save.essence += result.essence
            save.xp += result.xp
            // Level up
            while (save.xp >= save.level * 100) {
                save.xp -= save.level * 100
                save.level += 1
                save.essence += 200
            }
            val logText = result.log.joinToString("\n")
            val box = LinearLayout(this)
            box.orientation = LinearLayout.VERTICAL
            box.setPadding(dp(16), dp(16), dp(16), dp(16))
            box.addView(tv(if (result.victory) "🏆 انتصار!" else "💔 هزيمة", 20f, gold, true))
            val logView = tv(logText, 13f)
            logView.movementMethod = ScrollingMovementMethod()
            logView.layoutParams = LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT, dp(300))
            box.addView(logView)
            box.addView(tv("المكافأة: ${result.coins} عملة | ${result.essence} جوهر | ${result.xp} خبرة", 14f, gold, true))
            AlertDialog.Builder(this).setView(box).setPositiveButton("إغلاق") { _, _ ->
                refreshTopBar()
                showScreen("battle")
            }.show()
        }
        col.addView(fight)
        return scrollBox(col)
    }

    // ---------- Settings Screen ----------

    private fun settingsScreen(): View {
        val col = LinearLayout(this)
        col.orientation = LinearLayout.VERTICAL
        col.setPadding(dp(12), dp(12), dp(12), dp(12))

        col.addView(tv("⚙️ الإعدادات", 20f, gold, true))

        val aiBox = panelBox()
        aiBox.addView(tv("🤖 الراوي الذكي (نماذج مجانية)", 16f, gold, true))
        aiBox.addView(tv("اختر مزود الذكاء الاصطناعي الذي يولّد القصص والحوارات:", 13f, textDim))

        val providers = listOf(
            "local" to "محلي (دون إنترنت)",
            "pollinations" to "Pollinations (مجاني، بدون مفتاح)",
            "openrouter" to "OpenRouter (نماذج مجانية)",
            "groq" to "Groq (مجاني)",
            "gemini" to "Gemini (مجاني)"
        )
        val spinner = Spinner(this)
        val labels = providers.map { it.second }
        val adapter = ArrayAdapter(this, android.R.layout.simple_spinner_item, labels)
        adapter.setDropDownViewResource(android.R.layout.simple_spinner_dropdown_item)
        spinner.adapter = adapter
        val currentIdx = providers.indexOfFirst { it.first == save.aiProvider }.coerceAtLeast(0)
        spinner.setSelection(currentIdx)
        aiBox.addView(spinner)

        val modelInput = EditText(this)
        modelInput.hint = "اسم النموذج (اختياري)"
        modelInput.setText(save.aiModel)
        modelInput.setTextColor(textMain)
        modelInput.setHintTextColor(textDim)
        aiBox.addView(modelInput)

        val keyInput = EditText(this)
        keyInput.hint = "مفتاح API (اختياري لـ Pollinations)"
        keyInput.setText(save.aiKey)
        keyInput.setTextColor(textMain)
        keyInput.setHintTextColor(textDim)
        aiBox.addView(keyInput)

        val testText = tv("", 13f, textDim)
        val saveAi = btn("💾 حفظ إعدادات الذكاء الاصطناعي") {
            val idx = spinner.selectedItemPosition
            save.aiProvider = providers[idx].first
            save.aiModel = modelInput.text.toString().trim()
            save.aiKey = keyInput.text.toString().trim()
            Toast.makeText(this, "تم الحفظ! المزود: ${providers[idx].second}", Toast.LENGTH_LONG).show()
        }
        aiBox.addView(saveAi)
        val testBtn = btn("🔌 اختبار الاتصال") {
            testText.text = "⏳ جاري الاختبار..."
            AiNarrator.testConnection(save, object : AiNarrator.Callback {
                override fun onResult(text: String, fromAi: Boolean) {
                    testText.text = (if (fromAi) "✅ متصل بالذكاء الاصطناعي: " else "📝 وضع محلي: ") + text
                }
                override fun onError(message: String) {
                    testText.text = "❌ فشل: $message"
                }
            })
        }
        aiBox.addView(testBtn)
        aiBox.addView(testText)
        col.addView(aiBox)

        val danger = panelBox()
        danger.addView(tv("🗑️ منطقة الخطر", 16f, Color.RED, true))
        val reset = btn("حذف كل التقدم", Color.parseColor("#B71C1C"), Color.WHITE) {
            AlertDialog.Builder(this)
                .setTitle("حذف كل التقدم؟")
                .setMessage("سيتم حذف البطاقات والعملات والعالم نهائياً.")
                .setPositiveButton("نعم، احذف") { _, _ ->
                    save.resetAll()
                    selectedTeam.clear()
                    Toast.makeText(this, "تم حذف التقدم.", Toast.LENGTH_SHORT).show()
                    showScreen("gacha")
                }
                .setNegativeButton("إلغاء", null)
                .show()
        }
        danger.addView(reset)
        col.addView(danger)

        val about = panelBox()
        about.addView(tv("ℹ️ عن اللعبة", 16f, gold, true))
        about.addView(tv("لعبة بطاقات غاتشا نصية بعالم مفتوح مستوحاة من رواية «لورد الغامض» (Lord of the Mysteries) للكاتب Cuttlefish That Loves Diving. مشروع معجبين غير تجاري. جميع الشخصيات والأسماء ملك لأصحابها الأصليين.\n\nالميزات: استدعاء غاتشا بنظام الشفقة، عالم مفتوح باختيارات تغيّر العالم، معارك بطاقات، وراوي ذكاء اصطناعي مجاني يولّد القصص.", 13f, textDim))
        col.addView(about)

        return scrollBox(col)
    }
}
