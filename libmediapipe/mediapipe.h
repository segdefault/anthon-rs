#ifndef MEDIAPIPE_H
#define MEDIAPIPE_H

#include <string>

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
 
struct FFILandmark {
    float x, y;
};

struct FFIPacket {
    // size_t image_len;
    // uint8_t* image;

    size_t landmarks_len;
    FFILandmark* landmarks;
};

extern void* mediapipe_new();
extern void mediapipe_delete(void *mediapipe);
extern FFIPacket* mediapipe_process(void *mediapipe, uint8_t* data, size_t width, size_t height);
 
#ifdef __cplusplus
}
#endif

namespace mediapipe {

class Mediapipe {
public:
    // Create and initialize using provided graph
    // Returns nullptr if initialization failed
    static Mediapipe* Create(const std::string& graph);
    virtual ~Mediapipe(){}

    // Processes one frame and blocks until finished
    // Input data is expected to be ImageFormat::SRGB (24bits)
    // Returns nullptr if failed to run graph
    // Returns pointer to image whose size is the same as input image
    // and whose format is ImageFormat::SRGB
    // ImageFormat::SRGB is QImage::Format_RGB888 in Qt
    // Function does not take ownership of input data
    virtual FFIPacket* Process(uint8_t* data, size_t width, size_t height) = 0;
};

}

#endif
