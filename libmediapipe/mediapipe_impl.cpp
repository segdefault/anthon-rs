#include <cstdint>

#include "mediapipe_impl.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/port/file_helpers.h"
#include "mediapipe/framework/port/parse_text_proto.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/formats/image_frame.h"
#include "mediapipe/framework/formats/landmark.pb.h"

namespace mediapipe {
constexpr char kInputStream[] = "input_video";
constexpr char kLandmarksStream[] = "hand_landmarks";

MediapipeImpl::~MediapipeImpl() 
{
    LOG(INFO) << "Shutting down.";
    absl::Status status = m_graph.CloseInputStream(kInputStream);
    if (status.ok()){
    	absl::Status status1 = m_graph.WaitUntilDone();
        if (!status1.ok()) {
            LOG(INFO) << "Error in WaitUntilDone(): " << status1.ToString();
        }
    } else {
        LOG(INFO) << "Error in CloseInputStream(): " << status.ToString();
    }
}

absl::Status MediapipeImpl::Init(const std::string& graph) 
{
    LOG(INFO) << "Parsing graph config " << graph;
    mediapipe::CalculatorGraphConfig config = mediapipe::ParseTextProtoOrDie<mediapipe::CalculatorGraphConfig>(graph);

    LOG(INFO) << "Initialize the calculator graph.";
    MP_RETURN_IF_ERROR(m_graph.Initialize(config));

    LOG(INFO) << "Start running the calculator graph.";
    // ASSIGN_OR_RETURN(m_video_poller, m_graph.AddOutputStreamPoller(kOutputStream));
    MP_ASSIGN_OR_RETURN(m_landmarks_poller, m_graph.AddOutputStreamPoller(kLandmarksStream));
    MP_RETURN_IF_ERROR(m_graph.StartRun({}));

    return absl::OkStatus();
}

FFIPacket* MediapipeImpl::Process(uint8_t* data, size_t width, size_t height) 
{
    if (data == nullptr){
        LOG(INFO) << __FUNCTION__ << " input data is nullptr!";
        return nullptr;
    }

    size_t width_step = width * ImageFrame::ByteDepthForFormat(ImageFormat::SRGB) * ImageFrame::NumberOfChannelsForFormat(ImageFormat::SRGB);

    auto input_frame_for_input = absl::make_unique<ImageFrame>(ImageFormat::SRGB, width, height, width_step, 
                                                                (google::protobuf::uint8*)data, ImageFrame::PixelDataDeleter::kNone);

    m_frame_timestamp++;

    if (!m_graph.AddPacketToInputStream(kInputStream, mediapipe::Adopt(input_frame_for_input.release()).At(mediapipe::Timestamp(m_frame_timestamp))).ok()) {
        // LOG(INFO) << "Failed to add packet to input stream. Call m_graph.WaitUntilDone() to see error (or destroy Mediapipe object)";
        LOG(INFO) << "INSERTION ERROR: " << m_graph.WaitUntilDone().ToString();
        return nullptr;
    }

    // mediapipe::Packet video_packet;
    // if (!m_video_poller->Next(&video_packet)){
    //     // LOG(INFO) << "Poller didnt give me a packet, sorry. Call m_graph.WaitUntilDone() to see error (or destroy Mediapipe object). Error probably is that models are not available under mediapipe/models or mediapipe/modules";
    //     LOG(INFO) << "POLLING ERROR: " << m_graph.WaitUntilDone().ToString();
    //     return nullptr;
    // }

    int landmarks_queue_size = m_landmarks_poller->QueueSize();
    mediapipe::Packet landmarks_packet;
    if (landmarks_queue_size > 0 && !m_landmarks_poller->Next(&landmarks_packet)){
        // LOG(INFO) << "Poller didnt give me a packet, sorry. Call m_graph.WaitUntilDone() to see error (or destroy Mediapipe object). Error probably is that models are not available under mediapipe/models or mediapipe/modules";
        LOG(INFO) << "POLLING ERROR: " << m_graph.WaitUntilDone().ToString();
        return nullptr;
    }

    // const ImageFrame &output_frame = video_packet.Get<mediapipe::ImageFrame>();
    // size_t output_bytes = output_frame.PixelDataSizeStoredContiguously();


    // This could be optimized to not copy but return output_frame.PixelData()
    // uint8_t* out_data = new uint8_t[output_bytes];
    // output_frame.CopyToBuffer((uint8*)out_data, output_bytes);

    FFIPacket* ffi_packet = (FFIPacket*) malloc(sizeof(FFIPacket));
    // ffi_packet->image = out_data;
    // ffi_packet->image_len = output_bytes;
    ffi_packet->landmarks_len = 0;

    if (landmarks_queue_size > 0) {
        const std::vector<mediapipe::NormalizedLandmarkList> &landmarks = landmarks_packet.Get<std::vector<mediapipe::NormalizedLandmarkList>>();

        if (landmarks.size() > 0) {
            size_t landmarks_len = landmarks[0].landmark_size();
            FFILandmark* ffi_landmarks = (FFILandmark*) malloc(landmarks_len * sizeof(FFILandmark));

            for (size_t i = 0; i < landmarks_len; i++) 
            { 
                auto landmark = landmarks[0].landmark(i);

                ffi_landmarks[i].x = landmark.x(); 
                ffi_landmarks[i].y = landmark.y(); 
            }

            ffi_packet->landmarks_len = landmarks_len;
            ffi_packet->landmarks = ffi_landmarks;
        }
    }

    return ffi_packet; 
}

}
