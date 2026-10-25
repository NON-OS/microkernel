/*
 * qwenchat's vision tier: the image frame read from stdin, the projector
 * opened beside the model, and the first turn, which shows the model the
 * image ahead of the question. See qwenchat_vision.cpp.
 */
#pragma once
#include <vector>

#include "qwenchat.h"
#include "qwenvision_frame.h"

/* The frame from stdin: Ok with img empty for none, else the refusal. */
ImgWhy vision_frame(std::vector<unsigned char> &img);
/* The projector named by a.mmproj, once the model is open; false with c.err. */
bool vision_open(const ChatArgs &a, Chat &c);
/* A turn with the image ahead of `said`; img and said are wiped. */
bool vision_turn(const ChatArgs &a, Chat &c, std::vector<unsigned char> &img, std::string &said, Put put,
                 void *to, int &made);
void vision_close(Chat &c);
