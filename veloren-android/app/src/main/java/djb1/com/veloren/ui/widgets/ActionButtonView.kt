package djb1.com.veloren.ui.widgets

import android.content.Context
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Paint
import android.util.AttributeSet
import android.view.MotionEvent
import android.view.View

/**
 * Action Button Widget
 * Renders a tappable action button (Jump, Attack, etc.)
 */
class ActionButtonView @JvmOverloads constructor(
    context: Context,
    attrs: AttributeSet? = null
) : View(context, attrs) {

    // Button properties
    private val radius = 60f

    private val bgPaint = Paint().apply {
        color = Color.parseColor("#40FFFFFF")
        style = Paint.Style.FILL
        isAntiAlias = true
    }

    private val bgPressedPaint = Paint().apply {
        color = Color.parseColor("#80FFFFFF")
        style = Paint.Style.FILL
        isAntiAlias = true
    }

    private val borderPaint = Paint().apply {
        color = Color.parseColor("#FFFFFFFF")
        style = Paint.Style.STROKE
        strokeWidth = 3f
        isAntiAlias = true
    }

    private val textPaint = Paint().apply {
        color = Color.WHITE
        textSize = 28f
        isAntiAlias = true
        textAlign = Paint.Align.CENTER
    }

    var label: String = "BTN"
    var buttonPressed: Boolean = false
        private set

    var onAction: ((Boolean) -> Unit)? = null

    override fun onSizeChanged(w: Int, h: Int, oldw: Int, oldh: Int) {
        super.onSizeChanged(w, h, oldw, oldh)
        // Center the button
        setPadding(10, 10, 10, 10)
    }

    override fun onDraw(canvas: Canvas) {
        super.onDraw(canvas)

        val centerX = width / 2f
        val centerY = height / 2f

        // Draw background
        canvas.drawCircle(centerX, centerY, radius, if (buttonPressed) bgPressedPaint else bgPaint)
        canvas.drawCircle(centerX, centerY, radius, borderPaint)

        // Draw label
        canvas.drawText(label, centerX, centerY + textPaint.textSize / 3, textPaint)
    }

    override fun onTouchEvent(event: MotionEvent): Boolean {
        when (event.action) {
            MotionEvent.ACTION_DOWN -> {
                buttonPressed = true
                onAction?.invoke(true)
                invalidate()
                return true
            }
            MotionEvent.ACTION_UP, MotionEvent.ACTION_CANCEL -> {
                buttonPressed = false
                onAction?.invoke(false)
                invalidate()
                return true
            }
        }
        return super.onTouchEvent(event)
    }
}
