# Veloren Android - دليل المطور

## 🎯 نظرة عامة

مشروع لتحويل لعبة **Veloren** (لعبة voxel RPG مفتوحة المصدر مكتوبة بـ Rust) إلى تطبيق Android.

## 📁 البنية

```
veloren-android/
├── app/
│   ├── src/main/
│   │   ├── java/djb1/com/veloren/
│   │   │   ├── GameActivity.kt           ← النشاط الرئيسي
│   │   │   ├── VelorenGLSurfaceView.kt   ← عرض OpenGL ES
│   │   │   └── TouchInputHandler.kt      ← معالجة اللمس
│   │   ├── rust/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── lib.rs                ← ربط JNI
│   │   │       ├── render/               ← نظام الرسومات
│   │   │       ├── input.rs              ← نظام الإدخال
│   │   │       ├── assets.rs             ← نظام الأصول
│   │   │       └── world.rs              ← نظام العالم
│   │   └── AndroidManifest.xml
│   └── build.gradle.kts
├── build.sh                              ← سكريبت البناء
├── setup.sh                              ← سكريبت الإعداد
└── README.md
```

## 🚀 البدء السريع

### 1. تثبيت المتطلبات

```bash
# تثبيت Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# تشغيل سكريبت الإعداد
./setup.sh
```

### 2. تعيين المتغيرات

```bash
export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/26.0.10792818
```

### 3. البناء

```bash
./build.sh
```

### 4. التثبيت

```bash
adb install app/build/outputs/apk/debug/app-debug.apk
```

## 🏗️ البنية التفصيلية

### Android Layer (Kotlin)

#### GameActivity.kt
```kotlin
// النشاط الرئيسي للعبة
- إدارة دورة الحياة (onCreate, onResume, onPause)
- إعداد GLSurfaceView
- وضع ملء الشاشة وأفقي
```

#### VelorenGLSurfaceView.kt
```kotlin
// عرض OpenGL ES
- إعداد OpenGL ES 3.0
- معالجة أحداث اللمس
- Renderer
```

#### TouchInputHandler.kt
```kotlin
// معالجة اللمس
- Virtual Joysticks
- دعم متعدد اللمس
```

### Native Layer (Rust)

#### lib.rs
```rust
// ربط JNI بين Java و Rust
- nativeInit()
- nativeOnResume()
- nativeOnPause()
- nativeOnDestroy()
- nativeUpdate()
```

#### render/
```rust
// نظام الرسومات
- renderer.rs: محرك OpenGL ES
- shader.rs: نظام Shaders
- mesh.rs: نظام Meshes 3D
```

#### input.rs
```rust
// نظام الإدخال
- VirtualJoystick
- InputHandler
- معالجة اللمس المتعدد
```

#### assets.rs
```rust
// نظام الأصول
- تحميل الخامات
- تحميل النماذج
- تحميل الأصوات
- إدارة الذاكرة
```

#### world.rs
```rust
// نظام العالم
- Chunk system
- توليد التضاريس
- إدارة الكتل
```

## 📊 حالة المشروع

| المكون | الحالة | النسبة |
|--------|--------|--------|
| البنية الأساسية | ✅ مكتمل | 100% |
| JNI Bridge | ✅ مكتمل | 100% |
| OpenGL ES | ✅ أساسي | 40% |
| نظام الإدخال | ✅ مكتمل | 100% |
| نظام الأصول | ✅ أساسي | 30% |
| نظام العالم | ✅ أساسي | 20% |
| نقل Veloren | ⏳ لم يبدأ | 0% |
| **المجموع** | | **~30%** |

## 🔧 التطوير

### إضافة ميزة جديدة

1. **إضافة JNI function**
```rust
// في lib.rs
#[no_mangle]
pub extern "system" fn Java_djb1_com_veloren_ClassName_methodName(
    _env: JNIEnv,
    _class: JClass,
    param: jint,
) {
    // الكود هنا
}
```

2. **استدعاء من Kotlin**
```kotlin
external fun methodName(param: Int)
```

### اختبار التغييرات

```bash
# بناء
./build.sh

# تثبيت
adb install -r app/build/outputs/apk/debug/app-debug.apk

# عرض السجلات
adb logcat | grep -i veloren
```

## 🎮 التحكم

### Virtual Joysticks

```
┌─────────────────────────────────────┐
│                                     │
│   [Left Joystick]    [Right Joystick]│
│   الحركة             الكاميرا       │
│                                     │
│                      [Jump] [Attack]│
└─────────────────────────────────────┘
```

### الأزرار

- **العصا اليسرى**: الحركة (WASD)
- **العصا اليمنى**: الكاميرا
- **Jump**: القفز
- **Attack**: الهجوم

## 📝 الخطوات القادمة

### المرحلة 1: تحسين النظام الحالي
- [ ] اختبار JNI bridge
- [ ] تحسين معالجة اللمس
- [ ] إضافة أزرار على الشاشة

### المرحلة 2: نظام الرسومات
- [ ] تحويل Shaders من wgpu إلى GLSL
- [ ] عرض Cube تجريبي
- [ ] نظام الكاميرا

### المرحلة 3: العالم
- [ ] ربط world.rs بـ veloren-common
- [ ] توليد عالم voxel
- [ ] عرض التضاريس

### المرحلة 4: اللعب
- [ ] نظام الشخصيات
- [ ] نظام الحركات
- [ ] نظام الفيزياء

## ⚠️ التحديات

### 1. wgpu → OpenGL ES
Veloren يستخدم wgpu، Android يحتاج OpenGL ES

**الحل**: إعادة كتابة نظام الرسومات

### 2. التحكم
لوحة المفاتيح لا تعمل باللمس

**الحل**: Virtual Joysticks + أزرار

### 3. الأداء
الهاتف أضعف من الكمبيوتر

**الحل**:
- تقليل دقة العرض
- تقليل مسافة السحب
- استخدام LOD

### 4. الذاكرة
الهاتف لديه ذاكرة أقل

**الحل**:
- تحميل غير متزامن
- ضغط الأصول
- إدارة ذكية للذاكرة

## 🤝 المساهمة

المشروع في مرحلة التطوير المبكرة. أي مساعدة مرحب بها!

### أنواع المساهمات
- كتابة كود
- اختبار
- توثيق
- ترجمة
- تصميم

## 📄 الترخيص

GPL-3.0-or-later (نفس ترخيص Veloren الأصلي)

## 📞 التواصل

- [Veloren Discord](https://veloren.net/discord)
- [Veloren Zulip](https://veloren.net/zulip)

## 🔗 روابط مفيدة

- [Veloren الرسمي](https://veloren.net)
- [Veloren GitLab](https://gitlab.com/veloren/veloren)
- [Rust Android NDK](https://github.com/rust-windowing/android-ndk-rs)
- [OpenGL ES Docs](https://www.khronos.org/opengles/)
