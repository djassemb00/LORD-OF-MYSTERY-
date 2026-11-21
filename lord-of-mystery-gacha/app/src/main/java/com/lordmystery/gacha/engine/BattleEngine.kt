package com.lordmystery.gacha.engine

import com.lordmystery.gacha.data.CardDatabase
import com.lordmystery.gacha.data.GameCard
import kotlin.random.Random

data class Fighter(
    val card: GameCard,
    var hp: Int,
    var spirit: Int,
    val isPlayer: Boolean,
    var defending: Boolean = false
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

enum class Action { ATTACK, SKILL, DEFEND, FLEE }

/**
 * Interactive turn-based battle session.
 * Player acts one fighter at a time, then enemies act.
 */
class BattleSession(
    val players: MutableList<Fighter>,
    val foes: MutableList<Fighter>
) {
    val log = mutableListOf<String>()
    var round = 1
    var playerIndex = 0
    var over = false
    var victory = false

    init {
        log.add("⚔️ بدأت المعركة! الجولة 1 — اختر إجراءً للبطاقة الأولى.")
    }

    fun currentPlayer(): Fighter? = players.getOrNull(playerIndex)?.takeIf { it.alive }

    fun alivePlayers(): List<Fighter> = players.filter { it.alive }
    fun aliveFoes(): List<Fighter> = foes.filter { it.alive }

    /** Perform an action for the current player fighter. Returns log lines. */
    fun act(action: Action, targetIdx: Int = 0): List<String> {
        val lines = mutableListOf<String>()
        if (over) return lines
        val p = currentPlayer() ?: run { advance(); return act(action, targetIdx) }

        when (action) {
            Action.ATTACK -> {
                val targets = aliveFoes()
                if (targets.isEmpty()) { finish(true); return lines }
                val target = targets[targetIdx.coerceIn(0, targets.size - 1)]
                var dmg = (p.card.atk - target.card.def / 2).coerceAtLeast(5)
                if (p.defending) dmg = (dmg * 0.7).toInt().coerceAtLeast(1)
                target.hp -= dmg
                lines.add("${p.card.emoji} ${p.card.nameAr} هاجم ${target.card.nameAr} بقوة $dmg!")
                if (!target.alive) lines.add("💀 ${target.card.nameAr} سقط!")
            }
            Action.SKILL -> {
                if (p.spirit < p.card.skillCost) {
                    lines.add("⚠️ روحانية غير كافية للمهارة! (تحتاج ${p.card.skillCost})")
                    return lines
                }
                val targets = aliveFoes()
                if (targets.isEmpty()) { finish(true); return lines }
                val target = targets[targetIdx.coerceIn(0, targets.size - 1)]
                p.spirit -= p.card.skillCost
                var dmg = ((p.card.atk * p.card.skillPower / 100) - target.card.def / 2).coerceAtLeast(10)
                if (p.defending) dmg = (dmg * 0.7).toInt().coerceAtLeast(1)
                target.hp -= dmg
                lines.add("${p.card.emoji} ${p.card.nameAr} استخدم «${p.card.skillName}» على ${target.card.nameAr} بقوة $dmg!")
                if (!target.alive) lines.add("💀 ${target.card.nameAr} سقط!")
            }
            Action.DEFEND -> {
                p.defending = true
                p.spirit = (p.spirit + 15).coerceAtMost(p.card.spirit)
                lines.add("🛡️ ${p.card.nameAr} اتخذ وضعية دفاعية واستعاد 15 روحانية.")
            }
            Action.FLEE -> {
                if (Random.nextFloat() < 0.7f) {
                    lines.add("🏃 هربت بنجاح من المعركة!")
                    finish(false, fled = true)
                    return lines
                } else {
                    lines.add("❌ فشلت محاولة الهرب!")
                }
            }
        }
        p.defending = false
        log.addAll(lines)
        advance()
        return lines
    }

    private fun advance() {
        if (over) return
        if (aliveFoes().isEmpty()) { finish(true); return }
        playerIndex++
        // Skip dead players
        while (playerIndex < players.size && !players[playerIndex].alive) playerIndex++
        if (playerIndex >= players.size) {
            enemyTurn()
            if (over) return
            round++
            log.add("— الجولة $round —")
            playerIndex = 0
            while (playerIndex < players.size && !players[playerIndex].alive) playerIndex++
            if (playerIndex >= players.size) { finish(false); return }
        }
        if (alivePlayers().isEmpty()) finish(false)
    }

    private fun enemyTurn() {
        for (e in aliveFoes()) {
            val targets = alivePlayers()
            if (targets.isEmpty()) break
            val target = targets[Random.nextInt(targets.size)]
            val useSkill = e.spirit >= e.card.skillCost && Random.nextFloat() < 0.5f
            if (useSkill) {
                e.spirit -= e.card.skillCost
                var dmg = ((e.card.atk * e.card.skillPower / 100) - target.card.def / 2).coerceAtLeast(10)
                if (target.defending) dmg = (dmg * 0.5).toInt().coerceAtLeast(1)
                target.hp -= dmg
                log.add("${e.card.emoji} ${e.card.nameAr} استخدم «${e.card.skillName}» على ${target.card.nameAr} بقوة $dmg!")
            } else {
                var dmg = (e.card.atk - target.card.def / 2).coerceAtLeast(5)
                if (target.defending) dmg = (dmg * 0.5).toInt().coerceAtLeast(1)
                target.hp -= dmg
                log.add("${e.card.emoji} ${e.card.nameAr} هاجم ${target.card.nameAr} بقوة $dmg!")
            }
            if (!target.alive) log.add("💀 ${target.card.nameAr} سقط!")
        }
    }

    private fun finish(win: Boolean, fled: Boolean = false) {
        over = true
        victory = win
        if (fled) log.add("🏃 غادرت المعركة.")
        else if (win) log.add("🏆 انتصار!")
        else log.add("💔 هزيمة...")
    }

    fun result(): BattleResult {
        val coins: Int
        val essence: Int
        val xp: Int
        if (victory) {
            coins = 80 + foes.size * 40 + Random.nextInt(60)
            essence = 50 + foes.size * 30
            xp = 40 + foes.size * 25
        } else {
            coins = 20
            essence = 10
            xp = 10
        }
        return BattleResult(victory, coins, essence, xp, log.toList())
    }
}

/**
 * Battle helpers: enemy generation + legacy auto-simulate.
 */
object BattleEngine {

    fun generateEnemies(danger: Int, level: Int): List<GameCard> {
        val count = (2 + danger / 3).coerceIn(2, 4)
        val pool = CardDatabase.ALL.filter { it.rarity.ordinal <= 3 }
        val enemies = mutableListOf<GameCard>()
        repeat(count) {
            val maxRarityIdx = (danger / 3).coerceIn(0, 3)
            val candidates = pool.filter { it.rarity.ordinal <= maxRarityIdx }
            enemies.add(candidates[Random.nextInt(candidates.size)])
        }
        return enemies
    }

    fun newSession(playerTeam: List<GameCard>, enemies: List<GameCard>): BattleSession {
        return BattleSession(
            playerTeam.map { Fighter(it, it.hp, it.spirit, true) }.toMutableList(),
            enemies.map { Fighter(it, it.hp, it.spirit, false) }.toMutableList()
        )
    }

    /** Generate a boss: a high-rarity card with boosted stats. */
    fun generateBoss(danger: Int): List<GameCard> {
        val pool = CardDatabase.ALL.filter { it.rarity.ordinal >= 3 }
        val base = pool[Random.nextInt(pool.size)]
        val boost = 1.6f + danger * 0.05f
        val boss = base.copy(
            nameAr = "👑 زعيم: " + base.nameAr,
            atk = (base.atk * boost).toInt(),
            def = (base.def * boost).toInt(),
            hp = (base.hp * boost).toInt(),
            spirit = (base.spirit * boost).toInt()
        )
        // Boss + 1-2 minions
        val minions = generateEnemies((danger / 2).coerceAtLeast(1), 1).take(2)
        return listOf(boss) + minions
    }

    fun simulate(playerTeam: List<GameCard>, enemies: List<GameCard>): BattleResult {
        val session = newSession(playerTeam, enemies)
        var guard = 0
        while (!session.over && guard < 200) {
            guard++
            session.act(Action.ATTACK, 0)
        }
        return session.result()
    }
}
