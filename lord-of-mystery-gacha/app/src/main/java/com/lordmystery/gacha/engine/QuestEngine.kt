package com.lordmystery.gacha.engine

import java.text.SimpleDateFormat
import java.util.Date
import java.util.Locale

data class Quest(
    val id: String,
    val title: String,
    val desc: String,
    val target: Int,
    val rewardEssence: Int,
    val rewardCoins: Int
)

/**
 * Daily quests: reset every calendar day.
 */
object QuestEngine {

    val QUESTS: List<Quest> = listOf(
        Quest("pull", "المستدعي", "استدعِ 5 بطاقات", 5, 100, 50),
        Quest("explore", "المستكشف", "استكشف 3 لقاءات في العالم", 3, 80, 40),
        Quest("battle", "المحارب", "اربح معركة واحدة", 1, 120, 60),
        Quest("travel", "المسافر", "سافر إلى مكان آخر", 1, 60, 30),
        Quest("ai", "الراوي", "اسأل الراوي الذكي مرة", 1, 50, 20),
    )

    fun today(): String = SimpleDateFormat("yyyy-MM-dd", Locale.US).format(Date())

    fun ensureFresh(save: SaveManager) {
        if (save.questDate != today()) {
            save.questDate = today()
            save.resetQuests()
        }
    }

    fun progress(save: SaveManager, questId: String): Int {
        ensureFresh(save)
        return save.getQuestProgress(questId)
    }

    fun addProgress(save: SaveManager, questId: String, amount: Int = 1): Quest? {
        ensureFresh(save)
        val quest = QUESTS.firstOrNull { it.id == questId } ?: return null
        val before = save.getQuestProgress(questId)
        if (before >= quest.target) return null
        val after = (before + amount).coerceAtMost(quest.target)
        save.setQuestProgress(questId, after)
        if (after >= quest.target && before < quest.target) {
            save.essence += quest.rewardEssence
            save.coins += quest.rewardCoins
            return quest
        }
        return null
    }

    fun completed(save: SaveManager, quest: Quest): Boolean =
        progress(save, quest.id) >= quest.target
}
