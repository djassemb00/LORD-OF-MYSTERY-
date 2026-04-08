# Veloren Android - Detailed Documentation

## البنية الكاملة للمشروع

```
veloren-android/
├── app/
│   ├── src/main/
│   │   ├── java/djb1/com/veloren/
│   │   │   ├── GameActivity.kt           ← النشاط الرئيسي
│   │   │   ├── VelorenGLSurfaceView.kt   ← عرض OpenGL ES
│   │   │   └── TouchInputHandler.kt      ← معالجة اللمس
│   │   ├── rust/
│   │   │   ├── Cargo.toml                ← إعدادات Rust
│   │   │   └── src/
│   │   │       ├── lib.rs                ← ربط JNI
│   │   │       ├── render/
│   │   │       │   ├── mod.rs
│   │   │       │   ├── renderer.rs       ← محرك OpenGL ES
│   │   │       │   ├── shader.rs         ← نظام Shaders
│   │   │       │   └── mesh.rs           ← نظام Meshes
│   │   │       └── input.rs              ← نظام الإدخال
│   │   ├── res/
│   │   │   └── values/
│   │   │       └── strings.xml
│   │   └── AndroidManifest.xml
│   └── build.gradle.kts
├── bb/veloren-master/                    ← مصدر Veloren الأصلي
├── build.sh                              ← سكريبت البناء
├── setup.sh                              ← سكريبت الإعداد
└── README.md
```

## المكونات المكتملة

### 1. Android Java/Kotlin Layer
- ✅ GameActivity: النشاط الرئيسي مع دعم fullscreen
- ✅ VelorenGLSurfaceView: عرض OpenGL ES 3.0
- ✅ TouchInputHandler: معالجة أحداث اللمس
- ✅ Virtual Joysticks: عصا تحكم افتراضية

### 2. Rust Native Layer
- ✅ JNI Bridge: ربط بين Java و Rust
- ✅ GlRenderer: محرك OpenGL ES أساسي
- ✅ Shader System: نظام Shaders كامل
- ✅ Mesh System: دعم Meshes 3D
- ✅ Input Handler: نظام إدخال متعدد اللمس

## الخطوات القادمة

### المرحلة 1: نقل وحدات Veloren (شهر 1-2)

```rust
// مثال: نقل نظام الشخصيات
use veloren_common::character::Character;

struct Player {
    character: Character,
    position: Vec3<f32>,
    // ...
}
```

**المهام:**
1. استيراد `veloren-common`
2. نقل نظام ECS (Entity Component System)
3. نقل نظام توليد العالم
4. نقل نظام الحركات

### المرحلة 2: نظام الرسومات المتقدم (شهر 2-3)

```rust
// تحويل نظام الرسومات من wgpu إلى OpenGL ES
struct VelorenRenderer {
    shaders: ShaderProgram,
    meshes: HashMap<String, Mesh>,
    textures: HashMap<String, Texture>,
    // ...
}
```

**المهام:**
1. تحويل Shaders من wgpu إلى GLSL
2. تحميل النماذج 3D (voxel)
3. نظام الإضاءة
4. نظام الجسيمات (particles)
5. نظام السماء والطقس

### المرحلة 3: تحميل الأصول (شهر 3)

```rust
// تحميل الملفات من assets/
fn load_assets(&mut self, android_context: AndroidContext) {
    // تحميل النماذج
    // تحميل الخامات
    // تحميل الأصوات
    // تحميل الخطوط
}
```

**المهام:**
1. نظام تحميل الملفات من APK
2. ضغط الأصول
3. تحميل غير متزامن
4. إدارة الذاكرة

### المرحلة 4: الصوت (شهر 3-4)

```rust
// نظام الصوت
use cpal; // أو مكتبة أخرى

struct AudioEngine {
    music: Option<Sound>,
    sfx: Vec<Sound>,
    // ...
}
```

### المرحلة 5: الشبكة (شهر 4)

```rust
// دعم اللعب الجماعي
use veloren_common::net::Network;

struct NetworkManager {
    client: Client,
    // ...
}
```

## البناء والتشغيل

### 1. الإعداد

```bash
# تشغيل سكريبت الإعداد
chmod +x setup.sh
./setup.sh

# تعيين المتغيرات
export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/26.0.10792818
```

### 2. البناء

```bash
# بناء سريع
./build.sh

# أو يدوياً
cd app/src/main/rust
cargo build --target aarch64-linux-android --release
cd ../../../../..
./gradlew assembleDebug
```

### 3. التثبيت

```bash
# تثبيت عبر ADB
adb install app/build/outputs/apk/debug/app-debug.apk

# أو عبر Gradle
./gradlew installDebug
```

### 4. التشغيل

```bash
# تشغيل التطبيق
adb shell am start -n djb1.com.veloren/.GameActivity

# عرض السجلات
adb logcat | grep -i veloren
```

## التحديات والحلول

### التحدي 1: wgpu → OpenGL ES

**المشكلة:** Veloren يستخدم wgpu (Vulkan/Metal)
**الحل:** إعادة كتابة نظام الرسومات بـ OpenGL ES 3.0

### التحدي 2: التحكم

**المشكلة:** لوحة المفاتيح لا تعمل باللمس
**الحل:** Virtual Joysticks + أزرار على الشاشة

### التحدي 3: الأداء

**المشكلة:** الهاتف أضعف من الكمبيوتر
**الحل:**
- تقليل دقة العرض
- تقليل مسافة السحب (draw distance)
- تحسين Meshes
- استخدام LOD (Level of Detail)

### التحدي 4: الذاكرة

**المشكلة:** الهاتف لديه ذاكرة أقل
**الحل:**
- تحميل غير متزامن
- ضغط الأصول
- إدارة ذكية للذاكرة

## الترخيص

GPL-3.0-or-later (نفس ترخيص Veloren الأصلي)

## المساهمة

المشروع في مرحلة التطوير المبكرة. أي مساعدة مرحب بها!

## روابط مفيدة

- [Veloren الرسمي](https://veloren.net)
- [Veloren GitLab](https://gitlab.com/veloren/veloren)
- [Rust Android NDK](https://github.com/rust-windowing/android-ndk-rs)
- [OpenGL ES Docs](https://www.khronos.org/opengles/)
