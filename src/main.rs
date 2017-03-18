extern crate rusty_lamp_lib;

#[macro_use]
extern crate clap;

use clap::{ App, Arg, ArgMatches };
use rusty_lamp_lib::parser;
use rusty_lamp_lib::generator::spec::{Spec};
use rusty_lamp_lib::generator::typer::{Typer};
use rusty_lamp_lib::generator::ident_style::{IdentStyle, IdentConverter};

use std::env;
use std::path::Path;
use std::fs::{ File };
use std::io::{ Read, Write, BufWriter };
use std::io;
use std::sync::{Arc};

fn main() {
    println!("Hello, world!");

    let matches = App::new("rusty_lamp").about("Djinni implmentation in Rust")
        .author("Jonathan Rothberg")
        .arg(Arg::with_name("idl")
             .help("The IDF file with the type definitions, typically with extension \".djinni\".")
             .long("idl")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("fmt")
             .help("Format a Djinni file.")
             .long("fmt")
             // .takes_value(true)
             .required(false))
        .arg(Arg::with_name("include-path")
             .help("An include path to search for Djinni @import directives. Can specify multiple paths.")
             .takes_value(true)
             .multiple(true)
             .default_value("")
             .long("idl-include-path"))
        .arg(Arg::with_name("java-out")
             .help("The output for the Java files (Generator disabled if unspecified.)")
             .takes_value(true)
             .default_value("")
             .long("java-out"))
        .arg(Arg::with_name("java-package")
             .help("The package name to use for generated Java classes.")
             .takes_value(true)
             .long("java-package"))
        .arg(Arg::with_name("java-class-access-modifier")
             .help("The access modifier to use for generated Java classes.")
             .takes_value(true)
             .long("java-class-access-modifier")
             .default_value("public"))
        .arg(Arg::with_name("java-cpp-exception")
             .help("The type for translated C++ exceptions in Java")
             .takes_value(true)
             .long("java-cpp-exception")
             .default_value("java.lang.RuntimeException this is not checked"))
        .arg(Arg::with_name("java-annotation")
             .help("Java annotation (@Foo) to place on all generated Java classes")
             .takes_value(true)
             .long("java-annotation"))
        .arg(Arg::with_name("java-nullable-annotation")
             .help("Java annotation (@Nullable) to place all fields and return values that are optional.")
             .long("java-nullable-annotation"))
        .arg(Arg::with_name("java-nonnull-annotation")
             .help("Java annotation (@Notnull) to place on all fields and return values that are not optional")
             .long("java-nonnull-annotation"))
        .arg(Arg::with_name("java-use-final-for-record")
             .help("Whether generated Java classes for records should be marked 'final'")
             .long("java-use-final-for-record")
             .takes_value(true)
             .default_value("true"))

        // C++
        .arg(Arg::with_name("cpp-out")
             .help("The output folder for C++ files (Generator disabled if unspecified).")
             .long("cpp-out")
             .default_value(""))
        .arg(Arg::with_name("cpp-header-out")
             .help("The output folder for C++ header files")
             .long("cpp-header-out")
             .takes_value(true)
             .default_value("same as --cpp-out"))
        .arg(Arg::with_name("cpp-include-prefix")
             .help("The prefix for #includes of header files from C++ files.")
             .long("cpp-include-prefix")
             .takes_value(true))
        .arg(Arg::with_name("cpp-namespace")
             .help("The namespace name to use for generated C++ classes.")
             .long("cpp-namespace")
             .takes_value(true))
        .arg(Arg::with_name("cpp-ext")
             .help("The filename extension for C++ files.")
             .long("cpp-ext")
             .takes_value(true)
             .default_value("\"cpp\""))
        .arg(Arg::with_name("hpp-ext")
             .help("The filename extension for C++ header files.")
             .long("hpp-ext")
             .takes_value(true)
             .default_value("\"hpp\""))
        .arg(Arg::with_name("cpp-optional-template")
             .help("The template to use for optional values.")
             .long("cpp-optional-template")
             .takes_value(true)
             .default_value("\"std::optional\""))
        .arg(Arg::with_name("cpp-optional-header")
             .help("The header to use for optional values.")
             .long("cpp-optional-header")
             .takes_value(true)
             .default_value("\"<optional>\""))
        .arg(Arg::with_name("cpp-enum-hash-workaround")
             .help("Work around LWG-2148 by generating std::hash specializatins for C++ enums.")
             .long("cpp-enum-hash-workaround")
             .takes_value(true)
             .default_value("true"))
        .arg(Arg::with_name("cpp-nn-header")
             .help("The header to use for non-nullable pointers")
             .long("cpp-nn-header"))
        .arg(Arg::with_name("cpp-nn-type")
             .help("The type to use for non-nullable pointers (as a substitute for std::shared_ptr).")
             .long("cpp-nn-type"))
        .arg(Arg::with_name("cpp-nn-check-expression")
             .help("The expression to use for building non-nullable pointers")
             .long("cpp-nn-check-expression"))
        .arg(Arg::with_name("cpp-use-wide-strings")
             .help("Use wide strings in C++ code")
             .long("cpp-use-wide-strings")
             .takes_value(true)
             .default_value("false"))

        //JNI
        .arg(Arg::with_name("jni-out")
             .help("The folder for JNI C++ output files (Generator disabled if unspecified).")
             .long("jni-out")
             .default_value(""))
        .arg(Arg::with_name("jni-header-out")
             .help("The folder for JNI C++ header files.")
             .long("jni-header-out")
             .default_value("same as --jni-out"))
        .arg(Arg::with_name("jni-include-prefix")
             .help("The prefix for #includes of header files from JNI C++ files.")
             .long("jni-include-prefix")
             .takes_value(true))
        .arg(Arg::with_name("jni-namespace")
             .help("The namespace name to use for generated JNI C++ classes.")
             .long("jni-namespace")
             .takes_value(true))
        .arg(Arg::with_name("jni-base-lib-include-prefix")
             .help("The JNI base library's include path, relative to the JNI C++ classes.")
             .takes_value(true)
             .long("jni-base-lib-include-prefix"))
        //Objc
        .arg(Arg::with_name("objc-out")
             .help("The folder for Objective-C output files (Generator disabled if unspecified).")
             .default_value("")
             .long("objc-out"))
        .arg(Arg::with_name("objc-h-ext")
             .help("The filename extension for Objective-C[++] header files.")
             .long("objc-h-ext")
             .takes_value(true)
             .default_value("\"h\""))
        .arg(Arg::with_name("objc-type-prefix")
             .help("The prefix for Objective-C data types (usually two or three letters).")
             .long("objc-type-prefix")
             .takes_value(true))
        .arg(Arg::with_name("objc-include-prefix")
             .help("The prefix for #import of header files from Objective-C files.")
             .long("objc-include-prefix"))

       //Objective-C++
        .arg(Arg::with_name("objcpp-out")
             .help("The output folder for private Objective-C files (Generator disabled if unspecified).")
             .default_value("")
             .long("objcpp-out"))
        .arg(Arg::with_name("ojcpp-ext")
             .help("The filename extension for Objective-C++ files.")
             .long("objcpp-ext")
             .default_value("\"mm\""))
        .arg(Arg::with_name("objcpp-include-prefix")
             .help("The prefix for #import of header files from Objective-C++ files.")
             .long("objcpp-include-prefix")
             .takes_value(true))
        .arg(Arg::with_name("objcpp-include-cpp-prefix")
             .help("The prefix for #include of the main C++ header file from Objective-C++ files.")
             .long("objcpp-include-cpp-prefix")
             .takes_value(true))
        .arg(Arg::with_name("objcpp-include-objc-prefix")
             .help("The prefix for #import of the Objective-C header files from Objective-C++ files.")
             .long("objcpp-include-objc-prefix")
             .takes_value(true)
             .default_value("same as --objcpp-include-prefix"))
        .arg(Arg::with_name("cpp-extended-record-include-prefix")
             .help("The prefix path for #include of the extended record C++ header (.hpp) files.")
             .long("cpp-extended-record-include-prefix")
             .takes_value(true))
        .arg(Arg::with_name("objc-extended-record-include-prefix")
             .help("The prefix path for #import of the extended record Objective-C header (.h) files.")
             .long("objc-extended-record-include-prefix")
             .takes_value(true))
        .arg(Arg::with_name("objcpp-namespace")
             .help("The namespace name to use for generated Objective-C++ classes.")
             .long("objcpp-namespace")
             .takes_value(true))
        .arg(Arg::with_name("objc-base-lib-include-prefix")
             .help("The Objective-C++ base library's include path, relative to the Objective-C++ classes.")
             .takes_value(true)
             .long("objc-base-lib-include-prefix"))

        //YAML
        .arg(Arg::with_name("yaml-out")
             .help("The output folder for YAML files (Generator disabled if unspecified).")
             .long("yaml-out"))
        .arg(Arg::with_name("yaml-out-file")
             .help("If specified all types are merged into a single YAML file instead of generating one file per type (relative to --yaml-out)")
             .long("yaml-out-file"))
        .arg(Arg::with_name("yaml-prefix")
             .help("THe prefix to add to type names stored in YAML files.")
             .long("yaml-prefix")
             .default_value(""))

       // Misc
        .arg(Arg::with_name("list-in-files")
             .help("Optional file in which to write the list of input files parsed.")
             .long("list-in-files"))
        .arg(Arg::with_name("list-out-files")
             .help("Optional file in which to write the list of output files produced.")
             .long("list-out-files"))
        .arg(Arg::with_name("skip-generation")
             .help("Way of specifyiing if the file generation should be skipped.")
             .long("skip-generation")
             .default_value("false"))

       // Ident Style Flags
        .arg(Arg::with_name("ident-java-enum")
             .default_value("")
             .long("ident-java-enum"))
        .arg(Arg::with_name("ident-java-field")
             .default_value("")
             .long("ident-java-field"))
        .arg(Arg::with_name("ident-java-type")
             .default_value("")
             .long("ident-java-type"))

        .arg(Arg::with_name("ident-cpp-enum")
             .default_value("")
             .long("ident-cpp-enum"))
        .arg(Arg::with_name("ident-cpp-field")
             .default_value("")
             .long("ident-cpp-field"))
        .arg(Arg::with_name("ident-cpp-method")
             .default_value("")
             .long("ident-cpp-method"))
        .arg(Arg::with_name("ident-cpp-type")
             .default_value("")
             .long("ident-cpp-type"))
        .arg(Arg::with_name("ident-cpp-enum-type")
             .default_value("")
             .long("ident-cpp-enum-type"))
        .arg(Arg::with_name("ident-cpp-type-param")
             .default_value("")
             .long("ident-cpp-type-param"))
        .arg(Arg::with_name("ident-cpp-local")
             .default_value("")
             .long("ident-cpp-local"))
        .arg(Arg::with_name("ident-cpp-file")
             .default_value("")
             .long("ident-cpp-file"))

        .arg(Arg::with_name("ident-jni-class")
             .long("ident-jni-class")
             .default_value(""))
        .arg(Arg::with_name("ident-jni-file")
             .long("ident-jni-file")
             .default_value(""))

        .arg(Arg::with_name("ident-objc-enum")
             .default_value("")
             .long("ident-objc-enum"))
        .arg(Arg::with_name("ident-objc-field")
             .default_value("")
             .long("ident-objc-field"))
        .arg(Arg::with_name("ident-objc-method")
             .default_value("")
             .long("ident-objc-method"))
        .arg(Arg::with_name("ident-objc-type")
             .default_value("")
             .long("ident-objc-type"))
        .arg(Arg::with_name("ident-objc-type-param")
             .default_value("")
             .long("ident-objc-type-param"))
        .arg(Arg::with_name("ident-objc-local")
             .default_value("")
             .long("ident-objc-local"))
        .arg(Arg::with_name("ident-objc-file")
             .default_value("")
             .long("ident-objc-file")).get_matches();


    // let matches = clap_app!(rusty_lamp =>
    //                         (version: "0.1")
    //                         (author: "Jonathan Rothberg")
    //                         (about: "Djinni implementation in Rust")
    //                         (@arg IDL: +required "The IDL file with the type definitions, typically with extension \".djinni\".")
    //                         (@arg FMT: --fmt "Format a Djinni file")
    //                         (@arg include_path: --("idl-include-path") "An include path to search for Djinni @import directives. Can specify multiple paths.")
    //                         (@arg java_out: --("java-out") "The output for the Java files (Generator disabled if unspecified)")
    // ).get_matches();

    if let Some(ref ann) = matches.value_of("java_use_final_for_record") {
        println!("Package: {:?}", ann.parse::<bool>());
    }

    match matches.value_of("idl") {
        Some(i) => {
            match matches.occurrences_of("FMT") {
                1 => {
                    let cwd = match env::var("PWD") {
                        Ok(c) => c,
                        Err(_) => String::new()
                    };
                    println!("Working Directory: {}", cwd);

                    let mut file = File::open(i).unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    let mut stdout = io::stdout();

                    rusty_lamp_lib::process(contents, &mut stdout);
                },
                _ => {
                    println!("IDL: {}", i);
                    let idl_include_paths: Vec<&str> = match matches.values_of("id-include-path") {
                        Some(f) => f.collect(),
                        None => Vec::new()
                    };
                    let java_out_folder = matches.value_of("java-out");
                    println!("Java Out: {:?}", java_out_folder);
                    let java_package = matches.value_of("java-package");
                    // let ident_java_field = matches.value_of("ident-java-field").unwrap();
                    let java_class_access_modifier = matches.value_of("java-class-access-modifier").unwrap_or("");
                    let java_cpp_exception = matches.value_of("java-cpp-exception");
                    let java_annotation = matches.value_of("java-annotation");
                    let java_nullable_annotation = matches.value_of("java-nullable-annotation");
                    let java_nonnull_annotation = matches.value_of("java-nonnull-annotation");
                    let java_use_final_for_record = match matches.value_of("java-use-final-for-record") {
                        Some(f) => f.parse::<bool>().unwrap_or(false),
                        None => false
                    };

                    let cpp_out_folder = matches.value_of("cpp-out");
                    let cpp_header_out = matches.value_of("cpp-header-out");
                    let cpp_include_prefix = matches.value_of("cpp-include-prefix").unwrap_or("");
                    let cpp_namespace = matches.value_of("cpp-namespace").unwrap_or("");
                    let cpp_ext = matches.value_of("cpp-ext").unwrap_or("cpp");
                    let cpp_header_ext = matches.value_of("hpp-ext").unwrap_or("hpp");
                    let cpp_optional_template = matches.value_of("cpp-optional-template").unwrap_or("std::optional");
                    let cpp_optional_header = matches.value_of("cpp-optional-header").unwrap_or("<optional");
                    let cpp_enum_hash_workaround = match matches.value_of("cpp-enum-hash-workaround") {
                        Some(f) => f.parse::<bool>().unwrap_or(false),
                        None => false
                    };
                    let cpp_nn_header = matches.value_of("cpp-nn-header");
                    let cpp_nn_type = matches.value_of("cpp-nn-type");
                    let cpp_nn_check_expression = matches.value_of("cpp-nn-check-expression");
                    let cpp_use_wide_strings = match matches.value_of("cpp-use-wide-strings") {
                        Some(f) => f.parse::<bool>().unwrap_or(false),
                        None => false
                    };

                    let jni_out_folder = matches.value_of("jni-out");
                    let jni_header_out = matches.value_of("jni-header-out");
                    let jni_include_prefix = matches.value_of("jni-include-prefix").unwrap_or("");
                    let jni_include_cpp_prefix = matches.value_of("jni-include-cpp-prefix").unwrap_or("");
                    let jni_namespace = matches.value_of("jni-namespace").unwrap_or("djinni_generated");
                    let jni_base_lib_include_prefix = matches.value_of("jni-base-lib-include-prefix").unwrap_or("");


                    let objc_out_folder = matches.value_of("objc-out");
                    let objc_header_ext = matches.value_of("objc-h-ext").unwrap_or("h");
                    let objc_type_prefix = matches.value_of("objc-type-prefix").unwrap_or("");
                    println!("Objective-C Prefix: {}", objc_type_prefix);
                    let objc_include_prefix = matches.value_of("objc-include-prefix").unwrap_or("");

                    let objcpp_out_folder = matches.value_of("objcpp-out");
                    let objcpp_ext = matches.value_of("objcpp-ext").unwrap_or("mm");
                    let objcpp_include_prefix = matches.value_of("objcpp-include-prefix").unwrap_or("");
                    let objcpp_include_cpp_prefix = matches.value_of("objcpp-include-cpp-prefix").unwrap_or("");
                    let objcpp_include_objc_prefix_optional = matches.value_of("objcpp-include-objc-prefix");
                    let cpp_extended_record_include_prefix = matches.value_of("cpp-extended-record-include-prefix").unwrap_or("");
                    let objc_extended_record_include_prefix = matches.value_of("objc-extended-record-include-prefix").unwrap_or("");
                    let objcpp_namespace = matches.value_of("objcpp-namespace").unwrap_or("djinni_generated");
                    let objc_base_lib_include_prefix = matches.value_of("objc-base-lib-include-prefix").unwrap_or("");

                    let yaml_out = matches.value_of("yaml-out");
                    let yaml_out_file = matches.value_of("yaml-out-file");
                    let yaml_prefix = matches.value_of("yaml-prefix").unwrap_or("");

                    let list_in_files = matches.value_of("list-in-files");
                    let list_out_files = matches.value_of("list-out-files");
                    let skip_generation = match matches.value_of("skip-generation") {
                        Some(f) => f.parse::<bool>().unwrap_or(false),
                        None => false
                    };

                    // let ident_jni_class = matches.value_of("ident-jni-class").unwrap();
                    // let ident_jni_class_str = String::from(ident_jni_class);
                    // let java_ident_style = IdentStyle::build_ident_style(&ident_jni_class_str);
                    let mut ident_style = IdentStyle::new();
                    let mut cpp_ident_style = ident_style.cpp_style_default;
                    let mut java_ident_style = ident_style.java_style_default; 
                    cpp_ident_style.enm = Arc::new(IdentStyle::camel_lower);
                    get_ident_style(&matches, "ident-java-enum", |c| java_ident_style.enm = c);
                    get_ident_style(&matches, "ident-java-field", |c| java_ident_style.field = c);
                    get_ident_style(&matches, "ident-java-type", |c| java_ident_style.ty = c);

                    get_ident_style(&matches, "ident-cpp-enum", |c| cpp_ident_style.enm = c);
                    get_ident_style(&matches, "ident-cpp-field", |c| cpp_ident_style.field = c);
                    get_ident_style(&matches, "ident-cpp-method", |c| cpp_ident_style.field = c);
                    get_ident_style(&matches, "ident-cpp-type", |c| cpp_ident_style.field = c);
                    let mut cpp_type_enum_ident_style: Option<Arc<IdentConverter>> = None;
                    get_ident_style(&matches, "ident-cpp-enum-type", |c| cpp_type_enum_ident_style = Some(c));
                    get_ident_style(&matches, "ident-cpp-type-param", |c| cpp_ident_style.type_param = c);
                    get_ident_style(&matches, "ident-cpp-local", |c| cpp_ident_style.local = c);
                    let mut cpp_file_ident_style_optional: Option<Arc<IdentConverter>> = None;
                    get_ident_style(&matches, "ident-cpp-file", |c| cpp_file_ident_style_optional = Some(c));
                    let mut cpp_file_ident_style = cpp_file_ident_style_optional.unwrap_or(cpp_ident_style.ty.clone());

                    let mut jni_class_ident_style_optional: Option<Arc<IdentConverter>> = None;
                    get_ident_style(&matches, "ident-jni-class", |c| jni_class_ident_style_optional = Some(c));
                    let mut jni_file_ident_style: Option<Arc<IdentConverter>> = None;
                    get_ident_style(&matches, "ident-jni-file", |c| jni_file_ident_style = Some(c));

                    let mut objc_ident_style = ident_style.objc_style_default;
                    get_ident_style(&matches, "ident-objc-enum", |c| objc_ident_style.enm = c);
                    get_ident_style(&matches, "ident-objc-field", |c| objc_ident_style.field = c);
                    get_ident_style(&matches, "ident-objc-method", |c| objc_ident_style.method = c);
                    get_ident_style(&matches, "ident-objc-type", |c| objc_ident_style.ty = c);
                    get_ident_style(&matches, "ident-objc-type-param", |c| objc_ident_style.type_param = c);
                    get_ident_style(&matches, "ident-objc-local", |c| objc_ident_style.local = c);
                    let mut objc_file_ident_style_optional: Option<Arc<IdentConverter>> = None;
                    get_ident_style(&matches, "ident-objc-file", |c| objc_file_ident_style_optional = Some(c));

                    let cpp_header_out_folder = match cpp_header_out {
                        Some(_) => cpp_header_out,
                        None => cpp_out_folder.clone()
                    };

                    let jni_header_out_folder = match jni_header_out {
                        Some(_) => jni_header_out,
                        None => jni_out_folder.clone()
                    };

                    let jni_class_ident_style = match jni_class_ident_style_optional {
                        Some(jcis) => jcis,
                        None => cpp_ident_style.ty.clone()
                    };

                    let jni_base_lib_class_ident_style = jni_class_ident_style.clone();

                    let jni_file_ident_style = match jni_file_ident_style {
                        Some(s) => s,
                        None => cpp_file_ident_style.clone()
                    };

                    let mut objc_file_ident_style = match objc_file_ident_style_optional {
                        Some(s) => s,
                        None => objc_ident_style.ty.clone()
                    };

                    let objcpp_include_objc_prefix = match objcpp_include_objc_prefix_optional {
                        Some(s) => s,
                        None => objcpp_include_objc_prefix_optional.unwrap()
                    };

                    let prefix_func = objc_ident_style.ty.clone();
                    objc_ident_style.ty = IdentStyle::prefix(objc_type_prefix.into(), prefix_func);

                    objc_file_ident_style = IdentStyle::prefix(objc_type_prefix.into(), objc_file_ident_style.clone());

                    match cpp_type_enum_ident_style {
                        Some(s) => cpp_ident_style.enum_type = s.clone(),
                        None => {}
                    }
                                        
                    // let ident_jni_file = matches.value_of("ident-jni-file").unwrap();
                    println!("C++ Optional Template: {}", cpp_optional_template);

                    let out_file_list_writer = match list_out_files {
                        Some(p) => {
                            let p = Path::new(p);
                            let f = File::create(p.parent().unwrap()).expect("Unable to create file.");
                            // let mut bw = BufWriter::new(f);
                            Some(Arc::new(f))
                        },
                        None => None
                    };

                    let typer = Typer::new();
                    // let spec = Spec::new("generated-src".into(), "cpp".into(), typer);
                    let spec = Spec::new(typer,
                                         java_out_folder,
                                         java_package,
                                         java_class_access_modifier,
                                         java_ident_style,
                                         java_cpp_exception,
                                         java_annotation,
                                         java_nullable_annotation,
                                         java_nonnull_annotation,
                                         java_use_final_for_record,
                                         cpp_out_folder,
                                         cpp_header_out_folder,
                                         cpp_include_prefix,
                                         cpp_extended_record_include_prefix,
                                         cpp_namespace,
                                         cpp_ident_style,
                                         cpp_file_ident_style,
                                         cpp_optional_template,
                                         cpp_optional_header,
                                         cpp_enum_hash_workaround,
                                         cpp_nn_header,
                                         cpp_nn_type,
                                         cpp_nn_check_expression,
                                         cpp_use_wide_strings,
                                         jni_out_folder,
                                         jni_header_out_folder,
                                         jni_include_prefix,
                                         jni_include_cpp_prefix,
                                         jni_namespace,
                                         jni_class_ident_style,
                                         jni_file_ident_style,
                                         jni_base_lib_include_prefix,
                                         cpp_ext,
                                         cpp_header_ext,
                                         objc_out_folder,
                                         objcpp_out_folder,
                                         objc_ident_style,
                                         objc_file_ident_style,
                                         objcpp_ext,
                                         objc_header_ext,
                                         objc_include_prefix,
                                         objc_extended_record_include_prefix,
                                         objcpp_include_prefix,
                                         objcpp_include_cpp_prefix,
                                         objcpp_include_objc_prefix,
                                         objcpp_namespace,
                                         objc_base_lib_include_prefix,
                                         out_file_list_writer,
                                         skip_generation,
                                         yaml_out,
                                         yaml_out_file,
                                         yaml_prefix);
                    rusty_lamp_lib::compile(i.into(), &spec);
                }
            }
        },
        None => {

        }
    }
}

fn get_ident_style<S: AsRef<str>, F>(matches: &ArgMatches, flag_name: S, mut update: F) where F: FnMut(Arc<IdentConverter>){
    println!("Flag Name: {}", flag_name.as_ref());
    let m = matches.value_of(flag_name.as_ref());
    println!("M: {:?}", m);
    match matches.value_of(flag_name) {
        Some(f) if f.len() > 0 => {
            println!("Infering...{}", f);
            let ident_str = String::from(f);
            let ident_style = IdentStyle::build_ident_style(&ident_str);
            match ident_style {
                Some(func) => update(func),
                None => panic!("Invalid ident spec: {}", ident_str)
            }
        },
        _ => {}
    };
    
}
