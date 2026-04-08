package djb1.com.veloren.ui

import android.content.Context
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Paint
import android.graphics.RectF
import android.util.AttributeSet
import android.view.View

/**
 * Game HUD Overlay
 * Displays health, stamina, inventory, and debug info
 */
class GameHudOverlay @JvmOverloads constructor(
    context: Context,
    attrs: AttributeSet? = null
) : View(context, attrs) {

    // Paint objects
    private val healthBgPaint = Paint().apply {
        color = Color.parseColor("#40000000")
        style = Paint.Style.FILL
    }

    private val healthBarPaint = Paint().apply {
        color = Color.parseColor("#FF3333")
        style = Paint.Style.FILL
    }

    private val healthBarBgPaint = Paint().apply {
        color = Color.parseColor("#66FF3333")
        style = Paint.Style.FILL
    }

    private val staminaBgPaint = Paint().apply {
        color = Color.parseColor("#40000000")
        style = Paint.Style.FILL
    }

    private val staminaBarPaint = Paint().apply {
        color = Color.parseColor("#33FF33")
        style = Paint.Style.FILL
    }

    private val textPaint = Paint().apply {
        color = Color.WHITE
        textSize = 32f
        isAntiAlias = true
    }

    private val smallTextPaint = Paint().apply {
        color = Color.parseColor("#CCCCCC")
        textSize = 24f
        isAntiAlias = true
    }

    private val debugTextPaint = Paint().apply {
        color = Color.parseColor("#00FF00")
        textSize = 20f
        isAntiAlias = true
        typeface = android.graphics.Typeface.MONOSPACE
    }

    // Game state
    var health: Float = 100f
    var maxHealth: Float = 100f
    var stamina: Float = 100f
    var maxStamina: Float = 100f

    var fps: Int = 0
    var drawCalls: Int = 0
    var triangleCount: Int = 0

    var showDebug: Boolean = false

    // Bar dimensions
    private val barWidth = 400f
    private val barHeight = 30f
    private val barPadding = 10f
    private val barMargin = 20f
    private val barCornerRadius = 8f

    override fun onDraw(canvas: Canvas) {
        super.onDraw(canvas)

        val startX = barMargin
        var startY = barMargin + 40f // Status bar offset

        // Health bar
        drawBar(
            canvas = canvas,
            x = startX,
            y = startY,
            width = barWidth,
            height = barHeight,
            current = health,
            max = maxHealth,
            bgPaint = healthBgPaint,
            barBgPaint = healthBarBgPaint,
            barPaint = healthBarPaint,
            label = "HP"
        )

        // Stamina bar
        startY += barHeight + barPadding + barMargin
        drawBar(
            canvas = canvas,
            x = startX,
            y = startY,
            width = barWidth,
            height = barHeight,
            current = stamina,
            max = maxStamina,
            bgPaint = staminaBgPaint,
            barBgPaint = Paint().apply {
                color = Color.parseColor("#6633FF33")
                style = Paint.Style.FILL
            },
            barPaint = staminaBarPaint,
            label = "SP"
        )

        // Debug info
        if (showDebug) {
            startY += barHeight + barPadding + barMargin * 2
            drawDebugInfo(canvas, startX, startY)
        }

        // Controls hint
        drawControlsHint(canvas)
    }

    private fun drawBar(
        canvas: Canvas,
        x: Float,
        y: Float,
        width: Float,
        height: Float,
        current: Float,
        max: Float,
        bgPaint: Paint,
        barBgPaint: Paint,
        barPaint: Paint,
        label: String
    ) {
        val rect = RectF(x, y, x + width, y + height)

        // Background
        canvas.drawRoundRect(rect, barCornerRadius, barCornerRadius, bgPaint)

        // Bar background (colored)
        canvas.drawRoundRect(rect, barCornerRadius, barCornerRadius, barBgPaint)

        // Fill bar
        val fillWidth = (current / max) * (width - barPadding * 2)
        val fillRect = RectF(
            x + barPadding,
            y + barPadding,
            x + barPadding + fillWidth,
            y + height - barPadding
        )
        canvas.drawRoundRect(fillRect, barCornerRadius / 2, barCornerRadius / 2, barPaint)

        // Label
        canvas.drawText("$label ${current.toInt()}/${max.toInt()}", x + barPadding + 5, y + height / 2 + 10, textPaint)
    }

    private fun drawDebugInfo(canvas: Canvas, x: Float, y: Float) {
        var currentY = y
        val lineSpacing = 28f

        debugTextPaint.getTextBounds("FPS: 60", 0, 8, android.graphics.Rect())

        canvas.drawText("FPS: $fps", x, currentY, debugTextPaint)
        currentY += lineSpacing
        canvas.drawText("Draw Calls: $drawCalls", x, currentY, debugTextPaint)
        currentY += lineSpacing
        canvas.drawText("Triangles: $triangleCount", x, currentY, debugTextPaint)
        currentY += lineSpacing
        canvas.drawText("Memory: ${getMemoryUsage()}MB", x, currentY, debugTextPaint)
    }

    private fun drawControlsHint(canvas: Canvas) {
        val centerX = width / 2f
        val bottomY = height - 40f

        smallTextPaint.textAlign = Paint.Align.CENTER
        canvas.drawText(
            "Left: Move | Right: Look | Tap: Jump",
            centerX,
            bottomY,
            smallTextPaint
        )
        smallTextPaint.textAlign = Paint.Align.LEFT
    }

    private fun getMemoryUsage(): Int {
        val runtime = Runtime.getRuntime()
        val usedMemory = (runtime.totalMemory() - runtime.freeMemory()) / 1024 / 1024
        return usedMemory.toInt()
    }

    fun updateStats(fps: Int, drawCalls: Int, triangleCount: Int) {
        this.fps = fps
        this.drawCalls = drawCalls
        this.triangleCount = triangleCount
        invalidate()
    }

    fun updateHealth(health: Float, maxHealth: Float) {
        this.health = health
        this.maxHealth = maxHealth
        invalidate()
    }

    fun updateStamina(stamina: Float, maxStamina: Float) {
        this.stamina = stamina
        this.maxStamina = maxStamina
        invalidate()
    }

    fun toggleDebug() {
        showDebug = !showDebug
        invalidate()
    }
}
