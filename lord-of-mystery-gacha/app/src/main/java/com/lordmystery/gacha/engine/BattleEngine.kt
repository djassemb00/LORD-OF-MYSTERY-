package com.lordmystery.gacha.engine

import com.lordmystery.gacha.data.CardDatabase
import com.lordmystery.gacha.data.GameCard
import kotlin.random.Random

data class Fighter(
    val card: GameCard,
    var hp: Int,
    var spirit: Int,
    val isPlayer: Boolean
) {
    val maxHp: Int get() = card.hp
    val alive: Boolean get() = hp > 0
}

data class BattleResult(
    val victory: Boolean,
    val coins: Int,
    val essence: Int,
    val xp: Int,
    val log: List<String>
)

/**
 * Turn-based card battle engine.
 * Player team of 3 vs enemy team scaled by location danger.
 */
object BattleEngine {

    fun generateEnemies(danger: Int, level: Int): List<GameCard> {
        val count = (2 + danger / 3).coerceIn(2, 4)
        val pool = CardDatabase.ALL.filter { it.rarity.ordinal <= 3 }
        val enemies = mutableListOf<GameCard>()
        repeat(count) {
            // Higher danger -> higher rarity enemies
            val maxRarityIdx = (danger / 3).coerceIn(0, 3)
            val candidates = pool.filter { it.rarity.ordinal <= maxRarityIdx }
            enemies.add(candidates[Random.nextInt(candidates.size)])
        }
        return enemies
    }

    fun simulate(playerTeam: List<GameCard>, enemies: List<GameCard>): BattleResult {
        val log = mutableListOf<String>()
        val players = playerTeam.map { Fighter(it, it.hp, it.spirit, true) }.toMutableList()
        val foes = enemies.map { Fighter(it, it.hp, it.spirit, false) }.toMutableList()

        log.add("⚔️ بدأت المعركة! فريقك ضد ${enemies.size} أعداء.")

        var round = 0
        while (players.any { it.alive } && foes.any { it.alive } && round < 30) {
            round++
            log.add("— الجولة $round —")
            // Player turn
            for (p in players.filter { it.alive }) {
                val target = foes.filter { it.alive }.randomOrNull() ?: break
                val useSkill = p.spirit >= p.card.skillCost && Random.nextFloat() < 0.6f
                if (useSkill) {
                    p.spirit -= p.card.skillCost
                    val dmg = ((p.card.atk * p.card.skillPower / 100) - target.card.def / 2).coerceAtLeast(10)
                    target.hp -= dmg
                    log.add("${p.card.emoji} ${p.card.nameAr} استخدم «${p.card.skillName}» على ${target.card.nameAr} بقوة $dmg!")
                } else {
                    val dmg = (p.card.atk - target.card.def / 2).coerceAtLeast(5)
                    target.hp -= dmg
                    log.add("${p.card.emoji} ${p.card.nameAr} هاجم ${target.card.nameAr} بقوة $dmg.")
                }
                if (!target.alive) log.add("💀 ${target.card.nameAr} سقط!")
            }
            if (!foes.any { it.alive }) break
            // Enemy turn
            for (e in foes.filter { it.alive }) {
                val target = players.filter { it.alive }.randomOrNull() ?: break
                val useSkill = e.spirit >= e.card.skillCost && Random.nextFloat() < 0.5f
                if (useSkill) {
                    e.spirit -= e.card.skillCost
                    val dmg = ((e.card.atk * e.card.skillPower / 100) - target.card.def / 2).coerceAtLeast(10)
                    target.hp -= dmg
                    log.add("${e.card.emoji} ${e.card.nameAr} استخدم «${e.card.skillName}» على ${target.card.nameAr} بقوة $dmg!")
                } else {
                    val dmg = (e.card.atk - target.card.def / 2).coerceAtLeast(5)
                    target.hp -= dmg
                    log.add("${e.card.emoji} ${e.card.nameAr} هاجم ${target.card.nameAr} بقوة $dmg.")
                }
                if (!target.alive) log.add("💀 ${target.card.nameAr} سقط!")
            }
        }

        val victory = foes.none { it.alive }
        val coins: Int
        val essence: Int
        val xp: Int
        if (victory) {
            coins = 80 + enemies.size * 40 + Random.nextInt(60)
            essence = 50 + enemies.size * 30
            xp = 40 + enemies.size * 25
            log.add("🏆 انتصار! حصلت على $coins عملة و$essence جوهر غامض.")
        } else {
            coins = 20
            essence = 10
            xp = 10
            log.add("💔 هزيمة... لكنك حصلت على $coins عملة كتعويض.")
        }
        return BattleResult(victory, coins, essence, xp, log)
    }
}
