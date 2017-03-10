#! /usr/bin/env bash

rm -rf ./generated-src

base_dir=$(cd "`dirname "0"`" && pwd)
cpp_out="$base_dir/generated-src/cpp"
jni_out="$base_dir/generated-src/jni"
objc_out="$base_dir/generated-src/objc"
java_out="$base_dir/generated-src/java/com/bluebeam/core_gen"
java_package="com.bluebeam.core_gen"
namespace="core_gen"
objc_prefix="BB"
djinni_file="djinni-src/all.djinni"

cargo run -- --java-out $java_out \
   --java-package $java_package \
   --ident-java-field fooBar \
   \
    --cpp-optional-template "std::experimental::optional" \
    --cpp-optional-header "<optional/optional.hpp>" \
   \
   --cpp-out $cpp_out \
   --cpp-namespace $namespace \
   \
   --jni-out $jni_out \
   --ident-jni-class NativeFooBar \
   --ident-jni-file NativeFooBar \
   \
   --objc-out $objc_out \
   --objc-type-prefix $objc_prefix \
   \
   --objcpp-out $objc_out \
   \
   --idl $djinni_file

exit 0
