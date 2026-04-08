package djb1.com.veloren

import android.app.Activity
import android.content.pm.ActivityInfo
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.view.View
import android.view.WindowManager
import android.widget.FrameLayout
import djb1.com.veloren.ui.GameHudOverlay
import djb1.com.veloren.ui.widgets.ActionButtonView
import djb1.com.veloren.ui.widgets.VirtualJoystickView

/**
 * Main Game Activity for Veloren Android
 * Handles touch input and renders the game via OpenGL ES
 */
class GameActivity : Activity() {

    private lateinit var glSurfaceView: VelorenGLSurfaceView
    private lateinit var hudOverlay: GameHudOverlay
    private lateinit var leftJoystick: VirtualJoystickView
    private lateinit var rightJoystick: VirtualJoystickView
    private lateinit var jumpButton: ActionButtonView
    private lateinit var attackButton: ActionButtonView
    private lateinit var rootLayout: FrameLayout

    // FPS counter
    private var frameCount = 0
    private var lastFpsTime = 0L
    private var currentFps = 0
    private val handler = Handler(Looper.getMainLooper())

    companion object {
        init {
            // Load the native Rust library
            System.loadLibrary("veloren_android")
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Fullscreen and landscape
        window.setFlags(
            WindowManager.LayoutParams.FLAG_FULLSCREEN,
            WindowManager.LayoutParams.FLAG_FULLSCREEN
        )
        window.decorView.systemUiVisibility = (
            View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY
            or View.SYSTEM_UI_FLAG_HIDE_NAVIGATION
            or View.SYSTEM_UI_FLAG_FULLSCREEN
            or View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION
            or View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN
        )
        requestedOrientation = ActivityInfo.SCREEN_ORIENTATION_SENSOR_LANDSCAPE
        window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)

        // Create root layout
        rootLayout = FrameLayout(this)
        setContentView(rootLayout)

        // Create and set GL surface view
        glSurfaceView = VelorenGLSurfaceView(this)
        rootLayout.addView(glSurfaceView, FrameLayout.LayoutParams(
            FrameLayout.LayoutParams.MATCH_PARENT,
            FrameLayout.LayoutParams.MATCH_PARENT
        ))

        // Create HUD overlay
        hudOverlay = GameHudOverlay(this)
        rootLayout.addView(hudOverlay, FrameLayout.LayoutParams(
            FrameLayout.LayoutParams.MATCH_PARENT,
            FrameLayout.LayoutParams.MATCH_PARENT
        ))

        // Create virtual joysticks
        leftJoystick = VirtualJoystickView(this)
        val leftParams = FrameLayout.LayoutParams(400, 400)
        leftParams.setMargins(40, 0, 0, 40)
        leftParams.gravity = android.view.Gravity.BOTTOM or android.view.Gravity.START
        rootLayout.addView(leftJoystick, leftParams)

        rightJoystick = VirtualJoystickView(this)
        val rightParams = FrameLayout.LayoutParams(400, 400)
        rightParams.setMargins(0, 0, 40, 40)
        rightParams.gravity = android.view.Gravity.BOTTOM or android.view.Gravity.END
        rootLayout.addView(rightJoystick, rightParams)

        // Create action buttons
        jumpButton = ActionButtonView(this).apply {
            label = "JUMP"
            onAction = { pressed ->
                if (pressed) nativeJump()
            }
        }
        val jumpParams = FrameLayout.LayoutParams(200, 200)
        jumpParams.setMargins(0, 0, 200, 200)
        jumpParams.gravity = android.view.Gravity.BOTTOM or android.view.Gravity.END
        rootLayout.addView(jumpButton, jumpParams)

        attackButton = ActionButtonView(this).apply {
            label = "ATK"
            onAction = { pressed ->
                if (pressed) nativeAttack()
            }
        }
        val attackParams = FrameLayout.LayoutParams(200, 200)
        attackParams.setMargins(0, 0, 420, 200)
        attackParams.gravity = android.view.Gravity.BOTTOM or android.view.Gravity.END
        rootLayout.addView(attackButton, attackParams)

        // Initialize native game
        nativeInit(windowManager.defaultDisplay.width, windowManager.defaultDisplay.height)

        // Start FPS counter
        startFpsCounter()
    }

    private fun startFpsCounter() {
        lastFpsTime = System.currentTimeMillis()
        frameCount = 0

        handler.postDelayed(object : Runnable {
            override fun run() {
                frameCount++
                val now = System.currentTimeMillis()
                if (now - lastFpsTime >= 1000) {
                    currentFps = frameCount
                    frameCount = 0
                    lastFpsTime = now

                    // Update HUD
                    hudOverlay.updateStats(currentFps, 0, 0)
                }
                handler.postDelayed(this, 16) // ~60 FPS
            }
        }, 16)
    }

    override fun onResume() {
        super.onResume()
        glSurfaceView.onResume()
        nativeOnResume()
    }

    override fun onPause() {
        super.onPause()
        glSurfaceView.onPause()
        nativeOnPause()
        handler.removeCallbacksAndMessages(null)
    }

    override fun onDestroy() {
        super.onDestroy()
        nativeOnDestroy()
        handler.removeCallbacksAndMessages(null)
    }

    // Native method declarations
    external fun nativeInit(screenWidth: Int, screenHeight: Int)
    external fun nativeOnResume()
    external fun nativeOnPause()
    external fun nativeOnDestroy()
    external fun nativeUpdate(deltaTime: Float)
    external fun nativeJump()
    external fun nativeAttack()
}
