package com.lordmystery.gacha.engine

import android.content.Context
import android.content.SharedPreferences
import org.json.JSONObject

/**
 * Player state persisted as JSON in SharedPreferences.
 */
class SaveManager(context: Context) {

    private val prefs: SharedPreferences =
        context.getSharedPreferences("lord_gacha_save", Context.MODE_PRIVATE)

    var coins: Int
        get() = prefs.getInt("coins", 500)
        set(v) = prefs.edit().putInt("coins", v).apply()

    var essence: Int
        get() = prefs.getInt("essence", 3000)
        set(v) = prefs.edit().putInt("essence", v).apply()

    var pity: Int
        get() = prefs.getInt("pity", 0)
        set(v) = prefs.edit().putInt("pity", v).apply()

    var sanity: Int
        get() = prefs.getInt("sanity", 100)
        set(v) = prefs.edit().putInt("sanity", v.coerceIn(0, 100)).apply()

    var spirituality: Int
        get() = prefs.getInt("spirituality", 100)
        set(v) = prefs.edit().putInt("spirituality", v.coerceIn(0, 100)).apply()

    var locationId: String
        get() = prefs.getString("location", "tingen") ?: "tingen"
        set(v) = prefs.edit().putString("location", v).apply()

    var level: Int
        get() = prefs.getInt("level", 1)
        set(v) = prefs.edit().putInt("level", v).apply()

    var xp: Int
        get() = prefs.getInt("xp", 0)
        set(v) = prefs.edit().putInt("xp", v).apply()

    var lastDaily: Long
        get() = prefs.getLong("lastDaily", 0L)
        set(v) = prefs.edit().putLong("lastDaily", v).apply()

    // Reputations (-100..100)
    var repNighthawks: Int
        get() = prefs.getInt("repNighthawks", 0)
        set(v) = prefs.edit().putInt("repNighthawks", v.coerceIn(-100, 100)).apply()

    var repStorms: Int
        get() = prefs.getInt("repStorms", 0)
        set(v) = prefs.edit().putInt("repStorms", v.coerceIn(-100, 100)).apply()

    var repTarot: Int
        get() = prefs.getInt("repTarot", 0)
        set(v) = prefs.edit().putInt("repTarot", v.coerceIn(-100, 100)).apply()

    var repAurora: Int
        get() = prefs.getInt("repAurora", 0)
        set(v) = prefs.edit().putInt("repAurora", v.coerceIn(-100, 100)).apply()

    var repChurch: Int
        get() = prefs.getInt("repChurch", 0)
        set(v) = prefs.edit().putInt("repChurch", v.coerceIn(-100, 100)).apply()

    // Owned cards: cardId -> count
    fun getOwnedCards(): Map<String, Int> {
        val raw = prefs.getString("cards", "{}") ?: "{}"
        val obj = JSONObject(raw)
        val map = mutableMapOf<String, Int>()
        val keys = obj.keys()
        while (keys.hasNext()) {
            val k = keys.next()
            map[k] = obj.optInt(k, 0)
        }
        return map
    }

    fun addCard(cardId: String, count: Int = 1) {
        val map = getOwnedCards().toMutableMap()
        map[cardId] = (map[cardId] ?: 0) + count
        val obj = JSONObject()
        for ((k, v) in map) obj.put(k, v)
        prefs.edit().putString("cards", obj.toString()).apply()
    }

    fun hasCard(cardId: String): Boolean = (getOwnedCards()[cardId] ?: 0) > 0

    // Card levels: cardId -> level (1..10)
    fun getCardLevel(cardId: String): Int = prefs.getInt("lvl_$cardId", 1)

    fun setCardLevel(cardId: String, level: Int) {
        prefs.edit().putInt("lvl_$cardId", level.coerceIn(1, 10)).apply()
    }

    /** Upgrade cost in essence for next level. */
    fun upgradeCost(cardId: String): Int = getCardLevel(cardId) * 100

    // Daily quests: questId -> progress, and reset date
    var questDate: String
        get() = prefs.getString("questDate", "") ?: ""
        set(v) = prefs.edit().putString("questDate", v).apply()

    fun getQuestProgress(questId: String): Int = prefs.getInt("quest_$questId", 0)

    fun setQuestProgress(questId: String, value: Int) {
        prefs.edit().putInt("quest_$questId", value).apply()
    }

    fun addQuestProgress(questId: String, amount: Int = 1) {
        setQuestProgress(questId, getQuestProgress(questId) + amount)
    }

    fun resetQuests() {
        val editor = prefs.edit()
        for (key in prefs.all.keys) {
            if (key.startsWith("quest_")) editor.remove(key)
        }
        editor.apply()
    }

    // World flags
    fun setFlag(flag: String) {
        val flags = getFlags().toMutableSet()
        flags.add(flag)
        prefs.edit().putString("flags", flags.joinToString(",")).apply()
    }

    fun hasFlag(flag: String): Boolean = getFlags().contains(flag)

    fun getFlags(): Set<String> {
        val raw = prefs.getString("flags", "") ?: ""
        if (raw.isEmpty()) return emptySet()
        return raw.split(",").toSet()
    }

    // AI settings
    var aiProvider: String
        get() = prefs.getString("aiProvider", "local") ?: "local"
        set(v) = prefs.edit().putString("aiProvider", v).apply()

    var aiModel: String
        get() = prefs.getString("aiModel", "") ?: ""
        set(v) = prefs.edit().putString("aiModel", v).apply()

    var aiKey: String
        get() = prefs.getString("aiKey", "") ?: ""
        set(v) = prefs.edit().putString("aiKey", v).apply()

    fun resetAll() {
        prefs.edit().clear().apply()
    }
}
