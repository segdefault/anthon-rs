#ifndef MEDIAPIPE_IMPL_H
#define MEDIAPIPE_IMPL_H

#include "mediapipe.h"
#include "absl/status/status.h"
#include "mediapipe/framework/calculator_framework.h"

namespace mediapipe {

class MediapipeImpl : public Mediapipe {
public:
    MediapipeImpl(){}
    ~MediapipeImpl();

    absl::Status Init(const std::string& graph);

    FFIPacket* Process(uint8_t* data, size_t width, size_t height) override;

private:
    mediapipe::CalculatorGraph m_graph;
    // absl::StatusOr<OutputStreamPoller> m_video_poller;
    absl::StatusOr<OutputStreamPoller> m_landmarks_poller;
    size_t m_frame_timestamp = 0;
};

}

#endif
