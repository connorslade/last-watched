cmake_minimum_required(VERSION 3.16)

# Release service version, managed by release script
set(RELEASE_SERVICE_VERSION_MAJOR "25")
set(RELEASE_SERVICE_VERSION_MINOR "11")
set(RELEASE_SERVICE_VERSION_MICRO "70")

set(RELEASE_SERVICE_VERSION "${RELEASE_SERVICE_VERSION_MAJOR}.${RELEASE_SERVICE_VERSION_MINOR}.${RELEASE_SERVICE_VERSION_MICRO}")

project(last-watched VERSION ${RELEASE_SERVICE_VERSION})

set(QT_MIN_VERSION "6.7.0")
set(KF_MIN_VERSION "6.0.0")

# ECM setup
find_package(ECM ${KF_MIN_VERSION} CONFIG REQUIRED)
set(CMAKE_MODULE_PATH ${ECM_MODULE_PATH} ${CMAKE_SOURCE_DIR}/cmake)

include(QtVersionOption)
include(ECMSetupVersion)
include(KDEInstallDirs)
include(KDECMakeSettings)
include(KDECompilerSettings NO_POLICY_SCOPE)

include(ECMDeprecationSettings)
include(ECMOptionalAddSubdirectory)
include(ECMQtDeclareLoggingCategory)
include(FeatureSummary)

find_package(Qt6 ${QT_MIN_VERSION} REQUIRED COMPONENTS
    Core
    Widgets
    Network
    DBus
    Concurrent
)

find_package(KF6 ${KF_MIN_VERSION} REQUIRED COMPONENTS
    XmlGui
    I18n
    KIO
    TextWidgets
    Config
    CoreAddons
    Solid
)

find_package(DolphinVcs REQUIRED)
set_package_properties(DolphinVcs PROPERTIES
    DESCRIPTION "the Dolphin version control plugin library"
    URL "https://commits.kde.org/dolphin"
    TYPE REQUIRED
    PURPOSE "Provides the version control plugin interface."
)

ecm_set_disabled_deprecation_versions(
    QT 6.5
    KF 5.240
)

add_definitions(-DTRANSLATION_DOMAIN=\"lastwatchedplugin\")

kcoreaddons_add_plugin(lastwatchedplugin SOURCES src/lastwatchedplugin.cpp src/lastwatchedplugin.h INSTALL_NAMESPACE "dolphin/vcs")

target_link_libraries(lastwatchedplugin
    Qt6::Core
    Qt6::Widgets
    Qt6::Network
    KF6::I18n
    KF6::XmlGui
    KF6::KIOCore
    DolphinVcs
)


install(FILES com.connorcode.last-watched.metainfo.xml
        DESTINATION ${KDE_INSTALL_METAINFODIR})

ki18n_install(po)

feature_summary(WHAT ALL FATAL_ON_MISSING_REQUIRED_PACKAGES)

include(KDEClangFormat)
file(GLOB_RECURSE ALL_CLANG_FORMAT_SOURCE_FILES *.cpp *.h)
kde_clang_format(${ALL_CLANG_FORMAT_SOURCE_FILES})

include(KDEGitCommitHooks)
kde_configure_git_pre_commit_hook(CHECKS CLANG_FORMAT)
