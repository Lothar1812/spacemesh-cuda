cmake_minimum_required(VERSION 3.24 FATAL_ERROR)

if (NOT DEFINED CMAKE_CUDA_ARCHITECTURES)
    set(CMAKE_CUDA_ARCHITECTURES "all")
endif()

if (DEFINED CMAKE_BUILD_TYPE AND NOT CMAKE_BUILD_TYPE STREQUAL "")
    string(TOUPPER ${CMAKE_BUILD_TYPE} CMAKE_BUILD_TYPE)
else()
    set(CMAKE_BUILD_TYPE RELEASE)
endif()

project(spacemesh-cuda LANGUAGES CXX CUDA)
add_compile_options(/bigobj) # Anpassung für Windows

include(FetchContent)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

file(GLOB_RECURSE src_files LIST_DIRECTORIES false src/*.cu src/*.cpp src/*.c)

include_directories(include)
include_directories(src)

add_library(${PROJECT_NAME} ${src_files})

install(TARGETS ${PROJECT_NAME})

option(WITH_TEST "with test" OFF)

if(${WITH_TEST})
    enable_testing()
    set(TEST_EXE ${PROJECT_NAME}-test)

    if (DEFINED CMAKE_BUILD_TYPE AND NOT CMAKE_BUILD_TYPE STREQUAL "")
        string(TOUPPER ${CMAKE_BUILD_TYPE} CMAKE_BUILD_TYPE)
    else()
        set(CMAKE_BUILD_TYPE RELEASE)
    endif()

    if(${CMAKE_BUILD_TYPE} STREQUAL "RELEASE")
        set(NVCC_FLAGS /O2 /W3 /WX) # Anpassung für Windows: Optimierungen und Warnungen
        message(STATUS "${TEST_EXE} compile cuda code in release mode")
    else()
        set(NVCC_FLAGS /Od /W3 /WX /DEBUG) # Anpassung für Windows: Debug-Modus
        message(STATUS "${TEST_EXE} compile cuda code in debug mode")
    endif()

    #Google Unit Test
    option(BUILD_GMOCK "" OFF)
    option(INSTALL_GTEST "" OFF)
    FetchContent_Declare(
            googletest
            GIT_REPOSITORY https://github.com/google/googletest.git
            GIT_TAG release-1.12.1
    )
    FetchContent_MakeAvailable(googletest)

    file(GLOB_RECURSE test_files LIST_DIRECTORIES false test/*.cpp test/*.c test/*.cu)
    add_executable(${TEST_EXE} ${test_files})
    set_target_properties(${TEST_EXE} PROPERTIES CUDA_ARCHITECTURES "native")
    target_link_libraries(${TEST_EXE}
            PRIVATE ${PROJECT_NAME}
            PRIVATE gtest
            PRIVATE gtest_main)
    include(GoogleTest)
    set(NVCC_FLAGS ${NVCC_FLAGS})
    target_compile_options(${TEST_EXE} PRIVATE $<$<COMPILE_LANGUAGE:CUDA>:--forward-unknown-opts -Wall ${NVCC_FLAGS}>
            PRIVATE $<$<COMPILE_LANGUAGE:CXX>:-Wall>)
    add_test(NAME test_corr COMMAND ${TEST_EXE} --gtest_filter=*CheckResult*)
    add_test(NAME test_perf COMMAND ${TEST_EXE} --gtest_filter=*Perf*)
endif()
