package com.lordmystery.gacha.engine

import com.lordmystery.gacha.data.CardDatabase
import com.lordmystery.gacha.data.GameCard
import com.lordmystery.gacha.data.Rarity
import kotlin.random.Random

/**
 * Gacha summon engine with pity system.
 * Base rates: MYTHIC 3%, LEGENDARY 12%, EPIC 25%, RARE 35%, COMMON 25%.
 * Soft pity: +3% MYTHIC chance per pull after 70.
 * Hard pity: guaranteed MYTHIC at 90 pulls.
 */
object GachaEngine {

    const val COST_SINGLE = 160
    const val COST_TEN = 1600
    const val HARD_PITY = 90
    const val SOFT_PITY_START = 70

    fun pull(save: SaveManager, count: Int): List<GameCard> {
        val results = mutableListOf<GameCard>()
        repeat(count) {
            val card = rollOne(save)
            results.add(card)
            save.addCard(card.id)
            if (card.rarity == Rarity.MYTHIC) {
                save.pity = 0
            } else {
                save.pity = save.pity + 1
            }
        }
        return results
    }

    private fun rollOne(save: SaveManager): GameCard {
        val pity = save.pity
        val rarity: Rarity = when {
            pity + 1 >= HARD_PITY -> Rarity.MYTHIC
            else -> {
                var mythicChance = 3.0
                if (pity >= SOFT_PITY_START) {
                    mythicChance += (pity - SOFT_PITY_START + 1) * 3.0
                }
                val roll = Random.nextDouble(100.0)
                when {
                    roll < mythicChance -> Rarity.MYTHIC
                    roll < mythicChance + 12.0 -> Rarity.LEGENDARY
                    roll < mythicChance + 12.0 + 25.0 -> Rarity.EPIC
                    roll < mythicChance + 12.0 + 25.0 + 35.0 -> Rarity.RARE
                    else -> Rarity.COMMON
                }
            }
        }
        val pool = CardDatabase.byRarity(rarity)
        return pool[Random.nextInt(pool.size)]
    }

    /** Daily reward: 300 essence + 100 coins, once per 20 hours. */
    fun claimDaily(save: SaveManager): Boolean {
        val now = System.currentTimeMillis()
        if (now - save.lastDaily < 20 * 60 * 60 * 1000L) return false
        save.lastDaily = now
        save.essence += 300
        save.coins += 100
        return true
    }

    fun dailyAvailable(save: SaveManager): Boolean {
        return System.currentTimeMillis() - save.lastDaily >= 20 * 60 * 60 * 1000L
    }
}
