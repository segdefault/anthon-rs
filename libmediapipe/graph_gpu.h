static const std::string GRAPH = R"(
# CPU image. (ImageFrame)
input_stream: "input_video"
output_stream: "hand_landmarks"

node {
  calculator: "ConstantSidePacketCalculator"
  output_side_packet: "PACKET:model_complexity"
  node_options: {
    [type.googleapis.com/mediapipe.ConstantSidePacketCalculatorOptions]: {
      packet { int_value: 1 }
    }
  }
}

node {
  calculator: "ConstantSidePacketCalculator"
  output_side_packet: "PACKET:num_hands"
  node_options: {
    [type.googleapis.com/mediapipe.ConstantSidePacketCalculatorOptions]: {
      packet { int_value: 1 }
    }
  }
}


# Throttles the images flowing downstream for flow control. It passes through
# the very first incoming image unaltered, and waits for downstream nodes
# (calculators and subgraphs) in the graph to finish their tasks before it
# passes through another image. All images that come in while waiting are
# dropped, limiting the number of in-flight images in most part of the graph to
# 1. This prevents the downstream nodes from queuing up incoming images and data
# excessively, which leads to increased latency and memory usage, unwanted in
# real-time mobile applications. It also eliminates unnecessarily computation,
# e.g., the output produced by a node may get dropped downstream if the
# subsequent nodes are still busy processing previous inputs.
node {
  calculator: "FlowLimiterCalculator"
  input_stream: "input_video"
  input_stream: "FINISHED:hand_landmarks"
  input_stream_info: {
    tag_index: "FINISHED"
    back_edge: true
  }
  output_stream: "throttled_input_video"
}

node {
  calculator: "ImageFrameToGpuBufferCalculator"
  input_stream: "throttled_input_video"
  output_stream: "throttled_input_video_gpu"
}

node {
  calculator: "SetAlphaCalculator"
  input_stream: "IMAGE_GPU:throttled_input_video_gpu"
  output_stream: "IMAGE_GPU:throttled_input_video_gpu_rgba"
  node_options: {
    [type.googleapis.com/mediapipe.SetAlphaCalculatorOptions]: {
      alpha_value: 255
    }
  }
}

# Detects/tracks hand landmarks.
node {
  calculator: "HandLandmarkTrackingGpu"
  input_stream: "IMAGE:throttled_input_video_gpu_rgba"
  input_side_packet: "MODEL_COMPLEXITY:model_complexity"
  input_side_packet: "NUM_HANDS:num_hands"
  output_stream: "LANDMARKS:hand_landmarks"
  output_stream: "HANDEDNESS:handedness"
  output_stream: "PALM_DETECTIONS:palm_detections"
  output_stream: "HAND_ROIS_FROM_LANDMARKS:hand_rects_from_landmarks"
  output_stream: "HAND_ROIS_FROM_PALM_DETECTIONS:hand_rects_from_palm_detections"
}
)";