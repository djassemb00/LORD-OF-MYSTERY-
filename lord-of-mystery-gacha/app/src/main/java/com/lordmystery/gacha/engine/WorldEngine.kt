package com.lordmystery.gacha.engine

import com.lordmystery.gacha.data.Choice
import com.lordmystery.gacha.data.Encounter
import com.lordmystery.gacha.data.WorldDatabase
import kotlin.random.Random

/**
 * Open-world engine: travel, encounters, choices that change the world.
 */
object WorldEngine {

    fun travel(save: SaveManager, destId: String): String {
        val dest = WorldDatabase.location(destId) ?: return "المكان غير موجود."
        val current = WorldDatabase.location(save.locationId)
        if (current != null && !current.connections.contains(destId)) {
            return "لا يوجد طريق مباشر من ${current.nameAr} إلى ${dest.nameAr}."
        }
        save.locationId = destId
        // Travel costs some spirituality
        save.spirituality -= 5
        save.sanity -= 2
        return "سافرت إلى ${dest.nameAr}. ${dest.desc}"
    }

    fun randomEncounter(save: SaveManager): Encounter? {
        val pool = WorldDatabase.encountersFor(save.locationId)
        if (pool.isEmpty()) return null
        return pool[Random.nextInt(pool.size)]
    }

    fun applyChoice(save: SaveManager, choice: Choice): String {
        save.sanity += choice.sanity
        save.spirituality += choice.spirituality
        save.coins += choice.coins
        save.essence += choice.essence
        save.repNighthawks += choice.repNighthawks
        save.repStorms += choice.repStorms
        save.repTarot += choice.repTarot
        save.repAurora += choice.repAurora
        save.repChurch += choice.repChurch
        if (choice.flag.isNotEmpty()) save.setFlag(choice.flag)
        return choice.resultText
    }

    fun rest(save: SaveManager) {
        save.spirituality = 100
        save.sanity = (save.sanity + 20).coerceAtMost(100)
        save.coins -= 20
    }

    fun work(save: SaveManager) {
        save.coins += 60
        save.spirituality -= 15
        save.sanity -= 5
    }

    /** World summary for AI narrator / UI. */
    fun worldSummary(save: SaveManager): String {
        val loc = WorldDatabase.location(save.locationId)?.nameAr ?: "؟"
        return "المكان: $loc | المستوى: ${save.level} | العقل: ${save.sanity} | الروحانية: ${save.spirituality} | " +
            "سمعة فريق الليل: ${save.repNighthawks} | سمعة كنيسة العاصفة: ${save.repStorms} | " +
            "سمعة نادي التاروت: ${save.repTarot} | سمعة جماعة الفجر: ${save.repAurora} | " +
            "سمعة الكنيسة: ${save.repChurch} | أعلام العالم: ${save.getFlags().joinToString(",")}"
    }
}
