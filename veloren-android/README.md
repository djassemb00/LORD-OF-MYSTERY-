# Veloren Android

تطبيق Android للعبة Veloren - لعبة voxel RPG مفتوحة المصدر.

## البنية

```
veloren-android/
├── app/
│   ├── src/main/
│   │   ├── java/djb1/com/veloren/
│   │   │   └── GameActivity.kt          ← نشاط Android الرئيسي
│   │   ├── rust/
│   │   │   ├── Cargo.toml               ← مشروع Rust
│   │   │   └── src/
│   │   │       └── lib.rs               ← ربط JNI مع Android
│   │   ├── res/
│   │   │   └── values/
│   │   │       └── strings.xml
│   │   └── AndroidManifest.xml
│   └── build.gradle.kts
├── bb/veloren-master/                   ← مصدر Veloren الأصلي
├── build.sh                             ← سكريبت البناء
└── README.md
```

## المتطلبات

1. **Android SDK & NDK**
   - Android SDK 35+
   - Android NDK r26+

2. **Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **Rust Android Targets**
   ```bash
   rustup target add aarch64-linux-android armv7-linux-androideabi
   ```

## البناء

### الطريقة 1: استخدام السكريبت

```bash
chmod +x build.sh
./build.sh
```

### الطريقة 2: يدوياً

```bash
# 1. بناء مكتبة Rust
cd app/src/main/rust
cargo build --target aarch64-linux-android --release

# 2. نسخ المكتبة
cp target/aarch64-linux-android/release/libveloren_android.so \
   ../../jniLibs/arm64-v8a/

# 3. بناء APK
cd ../../../../..
./gradlew assembleDebug
```

## التثبيت

```bash
# تثبيت على الجهاز المتصل
adb install app/build/outputs/apk/debug/app-debug.apk

# أو عبر Gradle
./gradlew installDebug
```

## الحالة الحالية

- [x] هيكل مشروع Android
- [x] ربط JNI الأساسي
- [x] معالجة أحداث اللمس
- [ ] نظام الرسومات (OpenGL ES)
- [ ] نقل وحدات Veloren
- [ ] نظام التحكم باللمس
- [ ] تحميل الأصول (assets)

## التحديات القادمة

1. **نظام الرسومات**: تحويل wgpu إلى OpenGL ES
2. **نظام التحكم**: إعادة كتابة لللمس
3. **الأداء**: تحسين للهاتف
4. **الأصول**: تحميل الملفات

## الترخيص

GPL-3.0-or-later (نفس ترخيص Veloren)

## المساهمة

المشروع في مرحلة التطوير المبكرة. المساهمات مرحب بها!
