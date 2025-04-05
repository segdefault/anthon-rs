licenses(["notice"])

cc_binary(
    name="mediapipe_cpu",
    deps=[
        ":mediapipe_common",

        "//mediapipe/modules/hand_landmark:hand_landmark_tracking_cpu",
    ],
    linkopts=["-Wl,-soname,libmediapipe.so"],
    linkshared=1
)

cc_binary(
    name="mediapipe_gpu",
    deps=[
        ":mediapipe_common",

        "//mediapipe/gpu:image_frame_to_gpu_buffer_calculator",
        "//mediapipe/calculators/image:set_alpha_calculator",

        "//mediapipe/modules/hand_landmark:hand_landmark_tracking_gpu",
    ],
    linkopts=["-Wl,-soname,libmediapipe.so"],
    linkshared=1
)

cc_library(
    name="mediapipe_common",
    srcs=["mediapipe.cpp", "mediapipe_impl.cpp"],
    hdrs=["mediapipe.h", "mediapipe_impl.h", "graph_cpu.h", "graph_gpu.h"],
    visibility=["//visibility:public"],
    deps=[
        "//mediapipe/framework/port:parse_text_proto",
        "//mediapipe/framework:calculator_framework",
        "//mediapipe/framework/formats:image_frame",
        "//mediapipe/framework/formats:landmark_cc_proto",
    ],
    data = [
        "//mediapipe/modules/hand_landmark:hand_landmark_full.tflite",
        "//mediapipe/modules/palm_detection:palm_detection_full.tflite",
    ],
    alwayslink=1,
)
