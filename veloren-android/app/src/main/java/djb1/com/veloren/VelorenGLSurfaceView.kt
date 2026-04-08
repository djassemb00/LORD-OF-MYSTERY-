package djb1.com.veloren

import android.content.Context
import android.opengl.GLSurfaceView
import android.util.AttributeSet
import android.view.MotionEvent
import javax.microedition.khronos.egl.EGLConfig
import javax.microedition.khronos.opengles.GL10

/**
 * Custom GLSurfaceView for Veloren rendering
 */
class VelorenGLSurfaceView @JvmOverloads constructor(
    context: Context,
    attrs: AttributeSet? = null
) : GLSurfaceView(context, attrs) {

    private val touchInputHandler = TouchInputHandler()
    private var renderer: VelorenRenderer? = null

    init {
        // Set OpenGL ES 3.0
        setEGLContextClientVersion(3)
        
        // Create and set renderer
        renderer = VelorenRenderer(context)
        setRenderer(renderer)
        
        // Render only when dirty (better battery)
        renderMode = RENDERMODE_WHEN_DIRTY
    }

    override fun onTouchEvent(event: MotionEvent): Boolean {
        val action = event.actionMasked
        val pointerIndex = event.actionIndex
        val pointerId = event.getPointerId(pointerIndex)
        val x = event.getX(pointerIndex)
        val y = event.getY(pointerIndex)

        when (action) {
            MotionEvent.ACTION_DOWN, MotionEvent.ACTION_POINTER_DOWN -> {
                touchInputHandler.onTouchDown(pointerId, x, y)
            }
            MotionEvent.ACTION_MOVE -> {
                for (i in 0 until event.pointerCount) {
                    val id = event.getPointerId(i)
                    val moveX = event.getX(i)
                    val moveY = event.getY(i)
                    touchInputHandler.onTouchMove(id, moveX, moveY)
                }
            }
            MotionEvent.ACTION_UP, MotionEvent.ACTION_POINTER_UP -> {
                touchInputHandler.onTouchUp(pointerId)
            }
        }
        
        // Request new frame
        requestRender()
        
        return true
    }

    fun getTouchInputHandler(): TouchInputHandler {
        return touchInputHandler
    }
}

/**
 * OpenGL ES Renderer for Veloren
 */
class VelorenRenderer(private val context: Context) : GLSurfaceView.Renderer {

    override fun onSurfaceCreated(gl: GL10?, config: EGLConfig?) {
        // Set clear color (dark blue for now)
        gl?.glClearColor(0.1f, 0.1f, 0.2f, 1.0f)
        
        // Enable depth testing
        gl?.glEnable(GL10.GL_DEPTH_TEST)
        
        // Enable blending
        gl?.glEnable(GL10.GL_BLEND)
        gl?.glBlendFunc(GL10.GL_SRC_ALPHA, GL10.GL_ONE_MINUS_SRC_ALPHA)
        
        // Initialize native renderer
        nativeInitRenderer()
    }

    override fun onSurfaceChanged(gl: GL10?, width: Int, height: Int) {
        // Set viewport
        gl?.glViewport(0, 0, width, height)
        
        // Notify native renderer
        nativeOnResize(width, height)
    }

    override fun onDrawFrame(gl: GL10?) {
        // Clear screen
        gl?.glClear(GL10.GL_COLOR_BUFFER_BIT or GL10.GL_DEPTH_BUFFER_BIT)
        
        // Render frame
        nativeRenderFrame()
    }

    // Native methods
    external fun nativeInitRenderer()
    external fun nativeOnResize(width: Int, height: Int)
    external fun nativeRenderFrame()
}
