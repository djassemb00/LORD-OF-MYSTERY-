package com.lordmystery.gacha.engine

import com.lordmystery.gacha.data.CardDatabase

data class Achievement(
    val id: String,
    val title: String,
    val desc: String,
    val rewardEssence: Int
)

/**
 * Achievements unlocked by game milestones.
 */
object AchievementEngine {

    val ALL: List<Achievement> = listOf(
        Achievement("first_pull", "أول استدعاء", "استدع بطاقتك الأولى", 50),
        Achievement("collector_10", "جامع البطاقات", "اجمع 10 بطاقات مختلفة", 100),
        Achievement("collector_25", "خبير المجموعة", "اجمع 25 بطاقة مختلفة", 200),
        Achievement("mythic_one", "المختار", "احصل على بطاقة إلهية", 300),
        Achievement("mythic_five", "أسطورة حية", "اجمع 5 بطاقات إلهية", 500),
        Achievement("boss_slayer", "قاتل الزعماء", "اهزم زعيما واحدا", 200),
        Achievement("boss_master", "سيد العالم", "اهزم 5 زعماء", 500),
        Achievement("traveler", "المسافر", "زر 5 أماكن مختلفة", 100),
        Achievement("rich", "الثري", "اجمع 2000 عملة", 100),
        Achievement("level_5", "صاعد", "وصل إلى المستوى 5", 150),
        Achievement("level_10", "أسطوري", "وصل إلى المستوى 10", 400),
        Achievement("max_card", "الكمال", "ارفع بطاقة إلى المستوى 10", 300),
    )

    fun check(save: SaveManager): List<Achievement> {
        val unlocked = mutableListOf<Achievement>()
        val owned = save.getOwnedCards()
        val ownedCount = owned.size
        val mythicCount = owned.keys.count { CardDatabase.byId(it)?.rarity?.ordinal == 4 }
        val visited = save.getFlags().count { it.startsWith("visited_") }

        fun tryUnlock(id: String) {
            val ach = ALL.firstOrNull { it.id == id } ?: return
            if (save.unlockAchievement(id)) {
                save.essence += ach.rewardEssence
                unlocked.add(ach)
            }
        }

        if (ownedCount >= 1) tryUnlock("first_pull")
        if (ownedCount >= 10) tryUnlock("collector_10")
        if (ownedCount >= 25) tryUnlock("collector_25")
        if (mythicCount >= 1) tryUnlock("mythic_one")
        if (mythicCount >= 5) tryUnlock("mythic_five")
        if (save.bossKills >= 1) tryUnlock("boss_slayer")
        if (save.bossKills >= 5) tryUnlock("boss_master")
        if (visited >= 5) tryUnlock("traveler")
        if (save.coins >= 2000) tryUnlock("rich")
        if (save.level >= 5) tryUnlock("level_5")
        if (save.level >= 10) tryUnlock("level_10")
        // max card level checked on upgrade
        return unlocked
    }

    fun checkMaxCard(save: SaveManager, cardId: String): Achievement? {
        if (save.getCardLevel(cardId) >= 10) {
            val ach = ALL.firstOrNull { it.id == "max_card" } ?: return null
            if (save.unlockAchievement("max_card")) {
                save.essence += ach.rewardEssence
                return ach
            }
        }
        return null
    }
}
