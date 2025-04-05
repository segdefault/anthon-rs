#!/bin/sh

MEDIAPIPE_DIR=mediapipe_src
MEDIAPIPE_VERSION=v0.10.23
LIB_MEDIAPIPE_DIR=libmediapipe
ENABLE_GPU=0

if [ "$1" = "-g" ]; then
	ENABLE_GPU=1
fi

if [ ! -d "$MEDIAPIPE_DIR" ]; then
	git clone --branch "$MEDIAPIPE_VERSION" --depth 1 https://github.com/google-ai-edge/mediapipe.git "$MEDIAPIPE_DIR"
fi

ln -sf "$PWD/$LIB_MEDIAPIPE_DIR" "$MEDIAPIPE_DIR"
cd "$MEDIAPIPE_DIR"

if [ "$ENABLE_GPU" -eq 1 ]; then
	PROC_PFX="gpu"
else
	PROC_PFX="cpu"
fi

bazel build --jobs=12 -c opt `[ "$ENABLE_GPU" -eq 0 ] && echo --define MEDIAPIPE_DISABLE_GPU=1` //libmediapipe:mediapipe_"$PROC_PFX"

cp -f bazel-bin/libmediapipe/libmediapipe_"$PROC_PFX".so ../libmediapipe.so
cp -r bazel-bin/libmediapipe/libmediapipe_"$PROC_PFX".so.runfiles/mediapipe/mediapipe ..
cp -f mediapipe/modules/hand_landmark/handedness.txt ../mediapipe/modules/hand_landmark
