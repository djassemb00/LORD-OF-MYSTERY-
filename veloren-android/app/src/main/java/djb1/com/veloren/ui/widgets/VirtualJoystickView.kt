package djb1.com.veloren.ui.widgets

import android.content.Context
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Paint
import android.util.AttributeSet
import android.view.MotionEvent
import android.view.View
import kotlin.math.sqrt

/**
 * Virtual Joystick Widget
 * Renders a touch-controlled joystick for movement/camera
 */
class VirtualJoystickView @JvmOverloads constructor(
    context: Context,
    attrs: AttributeSet? = null
) : View(context, attrs) {

    // Joystick properties
    private val baseRadius = 120f
    private val knobRadius = 50f

    private val basePaint = Paint().apply {
        color = Color.parseColor("#30000000")
        style = Paint.Style.FILL
        isAntiAlias = true
    }

    private val baseBorderPaint = Paint().apply {
        color = Color.parseColor("#60FFFFFF")
        style = Paint.Style.STROKE
        strokeWidth = 3f
        isAntiAlias = true
    }

    private val knobPaint = Paint().apply {
        color = Color.parseColor("#80FFFFFF")
        style = Paint.Style.FILL
        isAntiAlias = true
    }

    private val knobBorderPaint = Paint().apply {
        color = Color.parseColor("#FFFFFFFF")
        style = Paint.Style.STROKE
        strokeWidth = 2f
        isAntiAlias = true
    }

    // Position
    var baseX = 0f
    var baseY = 0f
    private var knobX = 0f
    private var knobY = 0f

    // Output values (-1.0 to 1.0)
    var deltaX = 0f
        private set
    var deltaY = 0f
        private set

    var isActive = false
        private set

    private var touchId: Int? = null

    init {
        // Set initial position to center
        baseX = width / 2f
        baseY = height / 2f
        knobX = baseX
        knobY = baseY
    }

    override fun onSizeChanged(w: Int, h: Int, oldw: Int, oldh: Int) {
        super.onSizeChanged(w, h, oldw, oldh)
        baseX = w / 2f
        baseY = h / 2f
        knobX = baseX
        knobY = baseY
    }

    override fun onDraw(canvas: Canvas) {
        super.onDraw(canvas)

        // Draw base circle
        canvas.drawCircle(baseX, baseY, baseRadius, basePaint)
        canvas.drawCircle(baseX, baseY, baseRadius, baseBorderPaint)

        // Draw knob
        canvas.drawCircle(knobX, knobY, knobRadius, knobPaint)
        canvas.drawCircle(knobX, knobY, knobRadius, knobBorderPaint)
    }

    override fun onTouchEvent(event: MotionEvent): Boolean {
        val action = event.actionMasked
        val pointerIndex = event.actionIndex
        val pointerId = event.getPointerId(pointerIndex)
        val x = event.getX(pointerIndex)
        val y = event.getY(pointerIndex)

        when (action) {
            MotionEvent.ACTION_DOWN, MotionEvent.ACTION_POINTER_DOWN -> {
                // Check if touch is near the joystick
                val dist = sqrt((x - baseX) * (x - baseX) + (y - baseY) * (y - baseY))
                if (dist < baseRadius * 2 && touchId == null) {
                    touchId = pointerId
                    isActive = true
                    updateKnob(x, y)
                    return true
                }
            }
            MotionEvent.ACTION_MOVE -> {
                if (pointerId == touchId) {
                    updateKnob(x, y)
                    return true
                }
            }
            MotionEvent.ACTION_UP, MotionEvent.ACTION_POINTER_UP -> {
                if (pointerId == touchId) {
                    reset()
                    return true
                }
            }
        }
        return super.onTouchEvent(event)
    }

    private fun updateKnob(x: Float, y: Float) {
        var dx = x - baseX
        var dy = y - baseY

        val distance = sqrt(dx * dx + dy * dy)
        val maxDistance = baseRadius - knobRadius

        if (distance > maxDistance) {
            dx = (dx / distance) * maxDistance
            dy = (dy / distance) * maxDistance
        }

        knobX = baseX + dx
        knobY = baseY + dy

        // Normalize output
        deltaX = dx / maxDistance
        deltaY = dy / maxDistance

        invalidate()
    }

    fun reset() {
        knobX = baseX
        knobY = baseY
        deltaX = 0f
        deltaY = 0f
        isActive = false
        touchId = null
        invalidate()
    }

    fun setPosition(x: Float, y: Float) {
        baseX = x
        baseY = y
        knobX = x
        knobY = y
        invalidate()
    }
}
