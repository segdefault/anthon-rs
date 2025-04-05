#include "mediapipe.h"
#include "mediapipe_impl.h"

namespace mediapipe {

Mediapipe* Mediapipe::Create(const std::string& graph) 
{
    MediapipeImpl *mediapipe = new MediapipeImpl();
    absl::Status status = mediapipe->Init(graph);
    if (status.ok()){
        return mediapipe;
    }    
    else{
        LOG(INFO) << "Error initializing graph " << status.ToString();
        delete mediapipe;
        return nullptr;
    } 
}

}

#if MEDIAPIPE_DISABLE_GPU
#include "graph_cpu.h"
#else
#include "graph_gpu.h"
#endif  // MEDIAPIPE_DISABLE_GPU

extern void *mediapipe_new() {
    return mediapipe::Mediapipe::Create(GRAPH);
}

extern void mediapipe_delete(void *mediapipe) {
    mediapipe::Mediapipe *m = (mediapipe::Mediapipe*) mediapipe;

    delete m;
}

extern FFIPacket* mediapipe_process(void *mediapipe, uint8_t* data, size_t width, size_t height) {
    mediapipe::Mediapipe *m = (mediapipe::Mediapipe*) mediapipe;

    return  m->Process(data, width, height);
}
