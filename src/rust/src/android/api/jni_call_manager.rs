//
// Copyright 2019-2021 Signal Messenger, LLC
// SPDX-License-Identifier: AGPL-3.0-only
//

//! JNI Call Manager interface functions.
//!
//! Native JNI interfaces, called by
//! org.signal.ringrtc.CallManager objects.

use jni::objects::{JClass, JObject, JString};
use jni::strings::JavaStr;
use jni::sys::{jboolean, jbyteArray, jint, jlong, jobject};
use jni::JNIEnv;

use crate::android::android_platform::AndroidPlatform;
use crate::android::call_manager;
use crate::android::call_manager::AndroidCallManager;
use crate::android::error;
use crate::common::{CallConfig, CallMediaType, DataMode, DeviceId};
use crate::core::connection::Connection;
use crate::core::util::try_scoped;
use crate::core::{group_call, signaling};
use crate::webrtc;

use std::borrow::Cow;
use std::time::Duration;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcGetBuildInfo(
    env: JNIEnv,
    _class: JClass,
) -> jobject {
    match call_manager::get_build_info(&env) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
            0 as jobject
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcInitialize(
    env: JNIEnv,
    _class: JClass,
) {
    if let Err(e) = call_manager::initialize(&env) {
        error::throw_error(&env, e);
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcCreateCallManager(
    env: JNIEnv,
    _class: JClass,
    jni_call_manager: JObject,
) -> jlong {
    match call_manager::create_call_manager(&env, jni_call_manager) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
            0
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcCreatePeerConnection(
    env: JNIEnv,
    _object: JObject,
    peer_connection_factory: jlong,
    native_connection_borrowed: jlong,
    jni_rtc_config: JObject,
    jni_media_constraints: JObject,
) -> jlong {
    match call_manager::create_peer_connection(
        env,
        peer_connection_factory,
        webrtc::ptr::Borrowed::from_ptr(
            native_connection_borrowed as *mut Connection<AndroidPlatform>,
        ),
        jni_rtc_config,
        jni_media_constraints,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
            0
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcSetSelfUuid(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    uuid: jbyteArray,
) {
    match call_manager::set_self_uuid(&env, call_manager as *mut AndroidCallManager, uuid) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcCall(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    jni_remote: JObject,
    call_media_type: jint,
    local_device: jint,
) {
    match call_manager::call(
        &env,
        call_manager as *mut AndroidCallManager,
        jni_remote,
        CallMediaType::from_i32(call_media_type),
        local_device as DeviceId,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcProceed(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    call_id: jlong,
    jni_call_context: JObject,
    data_mode: jint,
    audio_levels_interval_millis: jint,
) {
    let audio_levels_interval = if audio_levels_interval_millis <= 0 {
        None
    } else {
        Some(Duration::from_millis(audio_levels_interval_millis as u64))
    };

    match call_manager::proceed(
        &env,
        call_manager as *mut AndroidCallManager,
        call_id,
        jni_call_context,
        CallConfig::default().with_data_mode(DataMode::from_i32(data_mode)),
        audio_levels_interval,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcMessageSent(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    call_id: jlong,
) {
    match call_manager::message_sent(call_manager as *mut AndroidCallManager, call_id) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcMessageSendFailure(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    call_id: jlong,
) {
    match call_manager::message_send_failure(call_manager as *mut AndroidCallManager, call_id) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcHangup(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
) {
    match call_manager::hangup(call_manager as *mut AndroidCallManager) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcCancelGroupRing(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    group_id: jbyteArray,
    ring_id: jlong,
    reason: jint,
) {
    match call_manager::cancel_group_ring(
        &env,
        call_manager as *mut AndroidCallManager,
        group_id,
        ring_id,
        reason,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcReceivedAnswer(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    call_id: jlong,
    remote_device: jint,
    opaque: jbyteArray,
    sender_identity_key: jbyteArray,
    receiver_identity_key: jbyteArray,
) {
    match call_manager::received_answer(
        &env,
        call_manager as *mut AndroidCallManager,
        call_id,
        remote_device as DeviceId,
        opaque,
        sender_identity_key,
        receiver_identity_key,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcReceivedOffer(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    call_id: jlong,
    jni_remote: JObject,
    remote_device: jint,
    opaque: jbyteArray,
    message_age_sec: jlong,
    call_media_type: jint,
    local_device: jint,
    jni_is_local_device_primary: jboolean,
    sender_identity_key: jbyteArray,
    receiver_identity_key: jbyteArray,
) {
    match call_manager::received_offer(
        &env,
        call_manager as *mut AndroidCallManager,
        call_id,
        jni_remote,
        remote_device as DeviceId,
        opaque,
        message_age_sec as u64,
        CallMediaType::from_i32(call_media_type),
        local_device as DeviceId,
        jni_is_local_device_primary == jni::sys::JNI_TRUE,
        sender_identity_key,
        receiver_identity_key,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcReceivedIceCandidates(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    call_id: jlong,
    remote_device: jint,
    jni_ice_candidates: JObject,
) {
    match call_manager::received_ice(
        &env,
        call_manager as *mut AndroidCallManager,
        call_id,
        remote_device as DeviceId,
        jni_ice_candidates,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcReceivedHangup(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    call_id: jlong,
    remote_device: jint,
    hangup_type: jint,
    device_id: jint,
) {
    match call_manager::received_hangup(
        call_manager as *mut AndroidCallManager,
        call_id,
        remote_device as DeviceId,
        signaling::HangupType::from_i32(hangup_type).unwrap_or(signaling::HangupType::Normal),
        device_id as DeviceId,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcReceivedBusy(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    call_id: jlong,
    remote_device: jint,
) {
    match call_manager::received_busy(
        call_manager as *mut AndroidCallManager,
        call_id,
        remote_device as DeviceId,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcReceivedCallMessage(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    sender_uuid: jbyteArray,
    sender_device_id: jint,
    local_device_id: jint,
    message: jbyteArray,
    message_age_sec: jlong,
) {
    match call_manager::received_call_message(
        &env,
        call_manager as *mut AndroidCallManager,
        sender_uuid,
        sender_device_id as DeviceId,
        local_device_id as DeviceId,
        message,
        message_age_sec as u64,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcReceivedHttpResponse(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    request_id: jlong,
    status_code: jint,
    body: jbyteArray,
) {
    match call_manager::received_http_response(
        &env,
        call_manager as *mut AndroidCallManager,
        request_id,
        status_code,
        body,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcHttpRequestFailed(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    request_id: jlong,
) {
    match call_manager::http_request_failed(call_manager as *mut AndroidCallManager, request_id) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcAcceptCall(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    call_id: jlong,
) {
    match call_manager::accept_call(call_manager as *mut AndroidCallManager, call_id) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcGetActiveConnection(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
) -> jobject {
    match call_manager::get_active_connection(call_manager as *mut AndroidCallManager) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
            0 as jobject
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcGetActiveCallContext(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
) -> jobject {
    match call_manager::get_active_call_context(call_manager as *mut AndroidCallManager) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
            0 as jobject
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcSetVideoEnable(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    enable: jboolean,
) {
    match call_manager::set_video_enable(call_manager as *mut AndroidCallManager, enable != 0) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcUpdateDataMode(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    data_mode: jint,
) {
    match call_manager::update_data_mode(
        call_manager as *mut AndroidCallManager,
        DataMode::from_i32(data_mode),
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcDrop(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    call_id: jlong,
) {
    match call_manager::drop_call(call_manager as *mut AndroidCallManager, call_id) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcReset(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
) {
    match call_manager::reset(call_manager as *mut AndroidCallManager) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcClose(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
) {
    match call_manager::close(call_manager as *mut AndroidCallManager) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

// Call Links

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcReadCallLink(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    sfu_url: JString,
    auth_credential_presentation: jbyteArray,
    root_key: jbyteArray,
    request_id: jlong,
) {
    match call_manager::read_call_link(
        &env,
        call_manager as *mut AndroidCallManager,
        sfu_url,
        auth_credential_presentation,
        root_key,
        request_id,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcCreateCallLink(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    sfu_url: JString,
    create_credential_presentation: jbyteArray,
    root_key: jbyteArray,
    admin_passkey: jbyteArray,
    call_link_public_params: jbyteArray,
    request_id: jlong,
) {
    match call_manager::create_call_link(
        &env,
        call_manager as *mut AndroidCallManager,
        sfu_url,
        create_credential_presentation,
        root_key,
        admin_passkey,
        call_link_public_params,
        request_id,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcUpdateCallLink(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    sfu_url: JString,
    auth_credential_presentation: jbyteArray,
    root_key: jbyteArray,
    admin_passkey: jbyteArray,
    new_name: JString,
    new_restrictions: jint,
    new_revoked: jint,
    request_id: jlong,
) {
    match call_manager::update_call_link(
        &env,
        call_manager as *mut AndroidCallManager,
        sfu_url,
        auth_credential_presentation,
        root_key,
        admin_passkey,
        new_name,
        new_restrictions,
        new_revoked,
        request_id,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

// Group Calls

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcPeekGroupCall(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    request_id: jlong,
    sfu_url: JString,
    membership_proof: jbyteArray,
    jni_serialized_group_members: jbyteArray,
) {
    match call_manager::peek_group_call(
        &env,
        call_manager as *mut AndroidCallManager,
        request_id,
        sfu_url,
        membership_proof,
        jni_serialized_group_members,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallManager_ringrtcPeekCallLinkCall(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    request_id: jlong,
    sfu_url: JString,
    auth_credential_presentation: jbyteArray,
    root_key: jbyteArray,
) {
    match call_manager::peek_call_link_call(
        &env,
        call_manager as *mut AndroidCallManager,
        request_id,
        sfu_url,
        auth_credential_presentation,
        root_key,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcCreateGroupCallClient(
    env: JNIEnv,
    _cls: JClass,
    call_manager: jlong,
    group_id: jbyteArray,
    sfu_url: JString,
    hkdf_extra_info: jbyteArray,
    audio_levels_interval_millis: jint,
    native_peer_connection_factory_borrowed_rc: jlong,
    native_audio_track_borrowed_rc: jlong,
    native_video_track_borrowed_rc: jlong,
) -> jlong {
    match call_manager::create_group_call_client(
        &env,
        call_manager as *mut AndroidCallManager,
        group_id,
        sfu_url,
        hkdf_extra_info,
        audio_levels_interval_millis,
        native_peer_connection_factory_borrowed_rc,
        native_audio_track_borrowed_rc,
        native_video_track_borrowed_rc,
    ) {
        Ok(v) => v as i64,
        Err(e) => {
            error::throw_error(&env, e);
            group_call::INVALID_CLIENT_ID as i64
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcCreateCallLinkCallClient(
    env: JNIEnv,
    _cls: JClass,
    call_manager: jlong,
    sfu_url: JString,
    auth_presentation: jbyteArray,
    call_link_bytes: jbyteArray,
    admin_passkey: jbyteArray,
    hkdf_extra_info: jbyteArray,
    audio_levels_interval_millis: jint,
    native_peer_connection_factory_borrowed_rc: jlong,
    native_audio_track_borrowed_rc: jlong,
    native_video_track_borrowed_rc: jlong,
) -> jlong {
    match call_manager::create_call_link_call_client(
        &env,
        call_manager as *mut AndroidCallManager,
        sfu_url,
        auth_presentation,
        call_link_bytes,
        admin_passkey,
        hkdf_extra_info,
        audio_levels_interval_millis,
        native_peer_connection_factory_borrowed_rc,
        native_audio_track_borrowed_rc,
        native_video_track_borrowed_rc,
    ) {
        Ok(v) => v as i64,
        Err(e) => {
            error::throw_error(&env, e);
            group_call::INVALID_CLIENT_ID as i64
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcDeleteGroupCallClient(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    client_id: jlong,
) {
    match call_manager::delete_group_call_client(
        call_manager as *mut AndroidCallManager,
        client_id as group_call::ClientId,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcConnect(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    client_id: jlong,
) {
    match call_manager::connect(
        call_manager as *mut AndroidCallManager,
        client_id as group_call::ClientId,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcJoin(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    client_id: jlong,
) {
    match call_manager::join(
        call_manager as *mut AndroidCallManager,
        client_id as group_call::ClientId,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcLeave(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    client_id: jlong,
) {
    match call_manager::leave(
        call_manager as *mut AndroidCallManager,
        client_id as group_call::ClientId,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcDisconnect(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    client_id: jlong,
) {
    match call_manager::disconnect(
        call_manager as *mut AndroidCallManager,
        client_id as group_call::ClientId,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcSetOutgoingAudioMuted(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    client_id: jlong,
    muted: bool,
) {
    match call_manager::set_outgoing_audio_muted(
        call_manager as *mut AndroidCallManager,
        client_id as group_call::ClientId,
        muted,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcSetOutgoingVideoMuted(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    client_id: jlong,
    muted: bool,
) {
    match call_manager::set_outgoing_video_muted(
        call_manager as *mut AndroidCallManager,
        client_id as group_call::ClientId,
        muted,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcRing(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    client_id: jlong,
    recipient: jbyteArray,
) {
    match call_manager::group_ring(
        &env,
        call_manager as *mut AndroidCallManager,
        client_id as group_call::ClientId,
        recipient,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcResendMediaKeys(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    client_id: jlong,
) {
    match call_manager::resend_media_keys(
        call_manager as *mut AndroidCallManager,
        client_id as group_call::ClientId,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcSetDataMode(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    client_id: jlong,
    data_mode: jint,
) {
    match call_manager::set_data_mode(
        call_manager as *mut AndroidCallManager,
        client_id as group_call::ClientId,
        DataMode::from_i32(data_mode),
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcRequestVideo(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    client_id: jlong,
    jni_rendered_resolutions: JObject,
    active_speaker_height: jint,
) {
    match call_manager::request_video(
        &env,
        call_manager as *mut AndroidCallManager,
        client_id as group_call::ClientId,
        jni_rendered_resolutions,
        active_speaker_height,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcApproveUser(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    client_id: jlong,
    other_user_id: jbyteArray,
) {
    match call_manager::approve_user(
        &env,
        call_manager as *mut AndroidCallManager,
        client_id as group_call::ClientId,
        other_user_id,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcDenyUser(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    client_id: jlong,
    other_user_id: jbyteArray,
) {
    match call_manager::deny_user(
        &env,
        call_manager as *mut AndroidCallManager,
        client_id as group_call::ClientId,
        other_user_id,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcRemoveClient(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    client_id: jlong,
    other_client_demux_id: jlong,
) {
    match call_manager::remove_client(
        call_manager as *mut AndroidCallManager,
        client_id as group_call::ClientId,
        other_client_demux_id,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcBlockClient(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    client_id: jlong,
    other_client_demux_id: jlong,
) {
    match call_manager::block_client(
        call_manager as *mut AndroidCallManager,
        client_id as group_call::ClientId,
        other_client_demux_id,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcSetGroupMembers(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    client_id: jlong,
    jni_serialized_group_members: jbyteArray,
) {
    match call_manager::set_group_members(
        &env,
        call_manager as *mut AndroidCallManager,
        client_id as group_call::ClientId,
        jni_serialized_group_members,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcSetMembershipProof(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    client_id: jlong,
    proof: jbyteArray,
) {
    match call_manager::set_membership_proof(
        &env,
        call_manager as *mut AndroidCallManager,
        client_id as group_call::ClientId,
        proof,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcReact(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    client_id: jlong,
    value: JString,
) {
    match call_manager::react(
        &env,
        call_manager as *mut AndroidCallManager,
        client_id as group_call::ClientId,
        value,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_GroupCall_ringrtcRaiseHand(
    env: JNIEnv,
    _object: JObject,
    call_manager: jlong,
    client_id: jlong,
    raise: bool,
) {
    match call_manager::raise_hand(
        call_manager as *mut AndroidCallManager,
        client_id as group_call::ClientId,
        raise,
    ) {
        Ok(v) => v,
        Err(e) => {
            error::throw_error(&env, e);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_signal_ringrtc_CallId_ringrtcFromEraId(
    env: JNIEnv,
    _class: JClass,
    era: JString,
) -> jlong {
    try_scoped(|| {
        // Avoid copying if we don't need to.
        let era_cesu8: JavaStr = env.get_string(era)?;
        let era_utf8: Cow<str> = Cow::from(&era_cesu8);
        Ok(group_call::RingId::from_era_id(&era_utf8).into())
    })
    .unwrap_or_else(|e| {
        error::throw_error(&env, e);
        0
    })
}
