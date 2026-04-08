package djb1.com.veloren

import android.graphics.PointF
import kotlin.math.sqrt

/**
 * Touch input handler for Veloren Android
 * Manages virtual joysticks and action buttons
 */
class TouchInputHandler {
    
    // Left joystick (movement)
    var leftJoystick = JoystickInput()
    
    // Right joystick (camera)
    var rightJoystick = JoystickInput()
    
    // Action buttons
    var jumpPressed = false
    var attackPressed = false
    var interactPressed = false
    
    // Touch zones
    private var leftJoystickId: Int? = null
    private var rightJoystickId: Int? = null
    
    // Screen dimensions
    private var screenWidth = 0f
    private var screenHeight = 0f
    
    fun setScreenDimensions(width: Float, height: Float) {
        screenWidth = width
        screenHeight = height
    }
    
    fun onTouchDown(pointerId: Int, x: Float, y: Float) {
        // Left half of screen = movement joystick
        if (x < screenWidth / 2 && leftJoystickId == null) {
            leftJoystickId = pointerId
            leftJoystick.setBasePoint(x, y)
        }
        // Right half = camera joystick or buttons
        else if (x >= screenWidth / 2 && rightJoystickId == null) {
            rightJoystickId = pointerId
            rightJoystick.setBasePoint(x, y)
        }
    }
    
    fun onTouchMove(pointerId: Int, x: Float, y: Float) {
        if (pointerId == leftJoystickId) {
            leftJoystick.updatePosition(x, y)
        } else if (pointerId == rightJoystickId) {
            rightJoystick.updatePosition(x, y)
        }
    }
    
    fun onTouchUp(pointerId: Int) {
        if (pointerId == leftJoystickId) {
            leftJoystick.reset()
            leftJoystickId = null
        } else if (pointerId == rightJoystickId) {
            rightJoystick.reset()
            rightJoystickId = null
        }
    }
    
    fun getMovementInput(): Pair<Float, Float> {
        return Pair(leftJoystick.deltaX, leftJoystick.deltaY)
    }
    
    fun getCameraInput(): Pair<Float, Float> {
        return Pair(rightJoystick.deltaX, rightJoystick.deltaY)
    }
}

/**
 * Virtual joystick input
 */
class JoystickInput {
    private var baseX = 0f
    private var baseY = 0f
    private var currentX = 0f
    private var currentY = 0f
    
    var deltaX = 0f
        private set
    var deltaY = 0f
        private set
    
    private val maxRadius = 100f
    
    fun setBasePoint(x: Float, y: Float) {
        baseX = x
        baseY = y
        currentX = x
        currentY = y
    }
    
    fun updatePosition(x: Float, y: Float) {
        currentX = x
        currentY = y
        
        var dx = currentX - baseX
        var dy = currentY - baseY
        
        // Clamp to max radius
        val distance = sqrt(dx * dx + dy * dy)
        if (distance > maxRadius) {
            dx = (dx / distance) * maxRadius
            dy = (dy / distance) * maxRadius
        }
        
        // Normalize to -1.0 to 1.0
        deltaX = dx / maxRadius
        deltaY = dy / maxRadius
    }
    
    fun reset() {
        deltaX = 0f
        deltaY = 0f
    }
}
