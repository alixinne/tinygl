#version 460 core

layout(location = 0) in vec3 uv;
layout(location = 0) out vec4 fragColor;

layout(location = 0) uniform float testFloat;
layout(location = 1) uniform vec2  testFloatVec2;
layout(location = 2) uniform vec3  testFloatVec3;
layout(location = 3) uniform vec4  testFloatVec4;

layout(location = 4) uniform double testDouble;
layout(location = 5) uniform dvec2  testDoubleVec2;
layout(location = 6) uniform dvec3  testDoubleVec3;
layout(location = 7) uniform dvec4  testDoubleVec4;

layout(location = 8)  uniform int   testInt;
layout(location = 9)  uniform ivec2 testIntVec2;
layout(location = 10) uniform ivec3 testIntVec3;
layout(location = 11) uniform ivec4 testIntVec4;

layout(location = 12) uniform uint  testUnsignedInt;
layout(location = 13) uniform uvec2 testUnsignedIntVec2;
layout(location = 14) uniform uvec3 testUnsignedIntVec3;
layout(location = 15) uniform uvec4 testUnsignedIntVec4;

layout(location = 16) uniform bool  testBool;
layout(location = 17) uniform bvec2 testBoolVec2;
layout(location = 18) uniform bvec3 testBoolVec3;
layout(location = 19) uniform bvec4 testBoolVec4;

layout(location = 20) uniform mat2   testFloatMat2;
layout(location = 21) uniform mat3   testFloatMat3;
layout(location = 22) uniform mat4   testFloatMat4;
layout(location = 23) uniform mat2x3 testFloatMat2x3;
layout(location = 24) uniform mat2x4 testFloatMat2x4;
layout(location = 25) uniform mat3x2 testFloatMat3x2;
layout(location = 26) uniform mat3x4 testFloatMat3x4;
layout(location = 27) uniform mat4x2 testFloatMat4x2;
layout(location = 28) uniform mat4x3 testFloatMat4x3;

layout(location = 29) uniform dmat2   testDoubleMat2;
layout(location = 30) uniform dmat3   testDoubleMat3;
layout(location = 31) uniform dmat4   testDoubleMat4;
layout(location = 32) uniform dmat2x3 testDoubleMat2x3;
layout(location = 33) uniform dmat2x4 testDoubleMat2x4;
layout(location = 34) uniform dmat3x2 testDoubleMat3x2;
layout(location = 35) uniform dmat3x4 testDoubleMat3x4;
layout(location = 36) uniform dmat4x2 testDoubleMat4x2;
layout(location = 37) uniform dmat4x3 testDoubleMat4x3;

layout(location = 38, binding = 0)  uniform sampler1D            testSampler1D;
layout(location = 39, binding = 1)  uniform sampler2D            testSampler2D;
layout(location = 40, binding = 2)  uniform sampler3D            testSampler3D;
layout(location = 41, binding = 3)  uniform samplerCube          testSamplerCube;
layout(location = 42, binding = 4)  uniform sampler1DShadow      testSampler1DShadow;
layout(location = 43, binding = 5)  uniform sampler2DShadow      testSampler2DShadow;
layout(location = 44, binding = 6)  uniform sampler1DArray       testSampler1DArray;
layout(location = 45, binding = 7)  uniform sampler2DArray       testSampler2DArray;
layout(location = 46, binding = 8)  uniform sampler1DArrayShadow testSampler1DArrayShadow;
layout(location = 47, binding = 9)  uniform sampler2DArrayShadow testSampler2DArrayShadow;
layout(location = 48, binding = 10) uniform sampler2DMS          testSampler2DMS;
layout(location = 49, binding = 11) uniform sampler2DMSArray     testSampler2DMSArray;
layout(location = 50, binding = 12) uniform samplerCubeShadow    testSamplerCubeShadow;
layout(location = 51, binding = 13) uniform samplerBuffer        testSamplerBuffer;
layout(location = 52, binding = 14) uniform sampler2DRect        testSampler2DRect;
layout(location = 53, binding = 15) uniform sampler2DRectShadow  testSampler2DRectShadow;
layout(location = 54, binding = 16) uniform isampler1D           testISampler1D;
layout(location = 55, binding = 17) uniform isampler2D           testISampler2D;
layout(location = 56, binding = 18) uniform isampler3D           testISampler3D;
layout(location = 57, binding = 19) uniform isamplerCube         testISamplerCube;
layout(location = 58, binding = 20) uniform isampler1DArray      testISampler1DArray;
layout(location = 59, binding = 21) uniform isampler2DArray      testISampler2DArray;
layout(location = 60, binding = 22) uniform isampler2DMS         testISampler2DMS;
layout(location = 61, binding = 23) uniform isampler2DMSArray    testISampler2DMSArray;
layout(location = 62, binding = 24) uniform isamplerBuffer       testISamplerBuffer;
layout(location = 63, binding = 25) uniform isampler2DRect       testISampler2DRect;
layout(location = 64, binding = 26) uniform usampler1D           testUSampler1D;
layout(location = 65, binding = 27) uniform usampler2D           testUSampler2D;
layout(location = 66, binding = 28) uniform usampler3D           testUSampler3D;
layout(location = 67, binding = 29) uniform usamplerCube         testUSamplerCube;
layout(location = 68, binding = 30) uniform usampler1DArray      testUSampler1DArray;
layout(location = 69, binding = 31) uniform usampler2DArray      testUSampler2DArray;
layout(location = 70, binding = 32) uniform usampler2DMS         testUSampler2DMS;
layout(location = 71, binding = 33) uniform usampler2DMSArray    testUSampler2DMSArray;
layout(location = 72, binding = 34) uniform usamplerBuffer       testUSamplerBuffer;
layout(location = 73, binding = 35) uniform usampler2DRect       testUSampler2DRect;

void main() {
    fragColor = vec4(uv, 1.);
}
