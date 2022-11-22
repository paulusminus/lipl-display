//! # DBus interface proxy for: `org.freedesktop.login1.Manager`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `Interface '/org/freedesktop/login1' from service 'org.freedesktop.login1' on system bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::PeerProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PropertiesProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.login1.Manager",
    default_service = "org.freedesktop.login1"
)]
trait Manager {
    /// ActivateSession method
    fn activate_session(&self, session_id: &str) -> zbus::Result<()>;

    /// ActivateSessionOnSeat method
    fn activate_session_on_seat(&self, session_id: &str, seat_id: &str) -> zbus::Result<()>;

    /// AttachDevice method
    fn attach_device(&self, seat_id: &str, sysfs_path: &str, interactive: bool)
        -> zbus::Result<()>;

    /// CanHalt method
    fn can_halt(&self) -> zbus::Result<String>;

    /// CanHibernate method
    fn can_hibernate(&self) -> zbus::Result<String>;

    /// CanHybridSleep method
    fn can_hybrid_sleep(&self) -> zbus::Result<String>;

    /// CanPowerOff method
    fn can_power_off(&self) -> zbus::Result<String>;

    /// CanReboot method
    fn can_reboot(&self) -> zbus::Result<String>;

    /// CanRebootParameter method
    fn can_reboot_parameter(&self) -> zbus::Result<String>;

    /// CanRebootToBootLoaderEntry method
    fn can_reboot_to_boot_loader_entry(&self) -> zbus::Result<String>;

    /// CanRebootToBootLoaderMenu method
    fn can_reboot_to_boot_loader_menu(&self) -> zbus::Result<String>;

    /// CanRebootToFirmwareSetup method
    fn can_reboot_to_firmware_setup(&self) -> zbus::Result<String>;

    /// CanSuspend method
    fn can_suspend(&self) -> zbus::Result<String>;

    /// CanSuspendThenHibernate method
    fn can_suspend_then_hibernate(&self) -> zbus::Result<String>;

    /// CancelScheduledShutdown method
    fn cancel_scheduled_shutdown(&self) -> zbus::Result<bool>;

    /// CreateSession method
    fn create_session(
        &self,
        uid: u32,
        pid: u32,
        service: &str,
        type_: &str,
        class: &str,
        desktop: &str,
        seat_id: &str,
        vtnr: u32,
        tty: &str,
        display: &str,
        remote: bool,
        remote_user: &str,
        remote_host: &str,
        properties: &[(&str, zbus::zvariant::Value<'_>)],
    ) -> zbus::Result<(
        String,
        zbus::zvariant::OwnedObjectPath,
        String,
        zbus::zvariant::OwnedFd,
        u32,
        String,
        u32,
        bool,
    )>;

    /// FlushDevices method
    fn flush_devices(&self, interactive: bool) -> zbus::Result<()>;

    /// GetSeat method
    fn get_seat(&self, seat_id: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetSession method
    fn get_session(&self, session_id: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetSessionByPID method
    fn get_session_by_pid(&self, pid: u32) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetUser method
    fn get_user(&self, uid: u32) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetUserByPID method
    fn get_user_by_pid(&self, pid: u32) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Halt method
    fn halt(&self, interactive: bool) -> zbus::Result<()>;

    /// HaltWithFlags method
    fn halt_with_flags(&self, flags: u64) -> zbus::Result<()>;

    /// Hibernate method
    fn hibernate(&self, interactive: bool) -> zbus::Result<()>;

    /// HibernateWithFlags method
    fn hibernate_with_flags(&self, flags: u64) -> zbus::Result<()>;

    /// HybridSleep method
    fn hybrid_sleep(&self, interactive: bool) -> zbus::Result<()>;

    /// HybridSleepWithFlags method
    fn hybrid_sleep_with_flags(&self, flags: u64) -> zbus::Result<()>;

    /// Inhibit method
    fn inhibit(
        &self,
        what: &str,
        who: &str,
        why: &str,
        mode: &str,
    ) -> zbus::Result<zbus::zvariant::OwnedFd>;

    /// KillSession method
    fn kill_session(&self, session_id: &str, who: &str, signal_number: i32) -> zbus::Result<()>;

    /// KillUser method
    fn kill_user(&self, uid: u32, signal_number: i32) -> zbus::Result<()>;

    /// ListInhibitors method
    fn list_inhibitors(&self) -> zbus::Result<Vec<(String, String, String, String, u32, u32)>>;

    /// ListSeats method
    fn list_seats(&self) -> zbus::Result<Vec<(String, zbus::zvariant::OwnedObjectPath)>>;

    /// ListSessions method
    fn list_sessions(
        &self,
    ) -> zbus::Result<Vec<(String, u32, String, String, zbus::zvariant::OwnedObjectPath)>>;

    /// ListUsers method
    fn list_users(&self) -> zbus::Result<Vec<(u32, String, zbus::zvariant::OwnedObjectPath)>>;

    /// LockSession method
    fn lock_session(&self, session_id: &str) -> zbus::Result<()>;

    /// LockSessions method
    fn lock_sessions(&self) -> zbus::Result<()>;

    /// PowerOff method
    fn power_off(&self, interactive: bool) -> zbus::Result<()>;

    /// PowerOffWithFlags method
    fn power_off_with_flags(&self, flags: u64) -> zbus::Result<()>;

    /// Reboot method
    fn reboot(&self, interactive: bool) -> zbus::Result<()>;

    /// RebootWithFlags method
    fn reboot_with_flags(&self, flags: u64) -> zbus::Result<()>;

    /// ReleaseSession method
    fn release_session(&self, session_id: &str) -> zbus::Result<()>;

    /// ScheduleShutdown method
    fn schedule_shutdown(&self, type_: &str, usec: u64) -> zbus::Result<()>;

    /// SetRebootParameter method
    fn set_reboot_parameter(&self, parameter: &str) -> zbus::Result<()>;

    /// SetRebootToBootLoaderEntry method
    fn set_reboot_to_boot_loader_entry(&self, boot_loader_entry: &str) -> zbus::Result<()>;

    /// SetRebootToBootLoaderMenu method
    fn set_reboot_to_boot_loader_menu(&self, timeout: u64) -> zbus::Result<()>;

    /// SetRebootToFirmwareSetup method
    fn set_reboot_to_firmware_setup(&self, enable: bool) -> zbus::Result<()>;

    /// SetUserLinger method
    fn set_user_linger(&self, uid: u32, enable: bool, interactive: bool) -> zbus::Result<()>;

    /// SetWallMessage method
    // fn set_wall_message(&self, wall_message: &str, enable: bool) -> zbus::Result<()>;

    /// Suspend method
    fn suspend(&self, interactive: bool) -> zbus::Result<()>;

    /// SuspendThenHibernate method
    fn suspend_then_hibernate(&self, interactive: bool) -> zbus::Result<()>;

    /// SuspendThenHibernateWithFlags method
    fn suspend_then_hibernate_with_flags(&self, flags: u64) -> zbus::Result<()>;

    /// SuspendWithFlags method
    fn suspend_with_flags(&self, flags: u64) -> zbus::Result<()>;

    /// TerminateSeat method
    fn terminate_seat(&self, seat_id: &str) -> zbus::Result<()>;

    /// TerminateSession method
    fn terminate_session(&self, session_id: &str) -> zbus::Result<()>;

    /// TerminateUser method
    fn terminate_user(&self, uid: u32) -> zbus::Result<()>;

    /// UnlockSession method
    fn unlock_session(&self, session_id: &str) -> zbus::Result<()>;

    /// UnlockSessions method
    fn unlock_sessions(&self) -> zbus::Result<()>;

    /// PrepareForShutdown signal
    #[dbus_proxy(signal)]
    fn prepare_for_shutdown(&self, start: bool) -> zbus::Result<()>;

    /// PrepareForSleep signal
    #[dbus_proxy(signal)]
    fn prepare_for_sleep(&self, start: bool) -> zbus::Result<()>;

    /// SeatNew signal
    #[dbus_proxy(signal)]
    fn seat_new(
        &self,
        seat_id: &str,
        object_path: zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// SeatRemoved signal
    #[dbus_proxy(signal)]
    fn seat_removed(
        &self,
        seat_id: &str,
        object_path: zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// SessionNew signal
    #[dbus_proxy(signal)]
    fn session_new(
        &self,
        session_id: &str,
        object_path: zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// SessionRemoved signal
    #[dbus_proxy(signal)]
    fn session_removed(
        &self,
        session_id: &str,
        object_path: zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// UserNew signal
    #[dbus_proxy(signal)]
    fn user_new(&self, uid: u32, object_path: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// UserRemoved signal
    #[dbus_proxy(signal)]
    fn user_removed(
        &self,
        uid: u32,
        object_path: zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// BlockInhibited property
    #[dbus_proxy(property)]
    fn block_inhibited(&self) -> zbus::Result<String>;

    /// BootLoaderEntries property
    #[dbus_proxy(property)]
    fn boot_loader_entries(&self) -> zbus::Result<Vec<String>>;

    /// DelayInhibited property
    #[dbus_proxy(property)]
    fn delay_inhibited(&self) -> zbus::Result<String>;

    /// Docked property
    #[dbus_proxy(property)]
    fn docked(&self) -> zbus::Result<bool>;

    /// EnableWallMessages property
    #[dbus_proxy(property)]
    fn enable_wall_messages(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_enable_wall_messages(&self, value: bool) -> zbus::Result<()>;

    /// HandleHibernateKey property
    #[dbus_proxy(property)]
    fn handle_hibernate_key(&self) -> zbus::Result<String>;

    /// HandleLidSwitch property
    #[dbus_proxy(property)]
    fn handle_lid_switch(&self) -> zbus::Result<String>;

    /// HandleLidSwitchDocked property
    #[dbus_proxy(property)]
    fn handle_lid_switch_docked(&self) -> zbus::Result<String>;

    /// HandleLidSwitchExternalPower property
    #[dbus_proxy(property)]
    fn handle_lid_switch_external_power(&self) -> zbus::Result<String>;

    /// HandlePowerKey property
    #[dbus_proxy(property)]
    fn handle_power_key(&self) -> zbus::Result<String>;

    /// HandleSuspendKey property
    #[dbus_proxy(property)]
    fn handle_suspend_key(&self) -> zbus::Result<String>;

    /// HoldoffTimeoutUSec property
    #[dbus_proxy(property)]
    fn holdoff_timeout_usec(&self) -> zbus::Result<u64>;

    /// IdleAction property
    #[dbus_proxy(property)]
    fn idle_action(&self) -> zbus::Result<String>;

    /// IdleActionUSec property
    #[dbus_proxy(property)]
    fn idle_action_usec(&self) -> zbus::Result<u64>;

    /// IdleHint property
    #[dbus_proxy(property)]
    fn idle_hint(&self) -> zbus::Result<bool>;

    /// IdleSinceHint property
    #[dbus_proxy(property)]
    fn idle_since_hint(&self) -> zbus::Result<u64>;

    /// IdleSinceHintMonotonic property
    #[dbus_proxy(property)]
    fn idle_since_hint_monotonic(&self) -> zbus::Result<u64>;

    /// InhibitDelayMaxUSec property
    #[dbus_proxy(property)]
    fn inhibit_delay_max_usec(&self) -> zbus::Result<u64>;

    /// InhibitorsMax property
    #[dbus_proxy(property)]
    fn inhibitors_max(&self) -> zbus::Result<u64>;

    /// KillExcludeUsers property
    #[dbus_proxy(property)]
    fn kill_exclude_users(&self) -> zbus::Result<Vec<String>>;

    /// KillOnlyUsers property
    #[dbus_proxy(property)]
    fn kill_only_users(&self) -> zbus::Result<Vec<String>>;

    /// KillUserProcesses property
    #[dbus_proxy(property)]
    fn kill_user_processes(&self) -> zbus::Result<bool>;

    /// LidClosed property
    #[dbus_proxy(property)]
    fn lid_closed(&self) -> zbus::Result<bool>;

    /// NAutoVTs property
    #[dbus_proxy(property)]
    fn nauto_vts(&self) -> zbus::Result<u32>;

    /// NCurrentInhibitors property
    #[dbus_proxy(property)]
    fn ncurrent_inhibitors(&self) -> zbus::Result<u64>;

    /// NCurrentSessions property
    #[dbus_proxy(property)]
    fn ncurrent_sessions(&self) -> zbus::Result<u64>;

    /// OnExternalPower property
    #[dbus_proxy(property)]
    fn on_external_power(&self) -> zbus::Result<bool>;

    /// PreparingForShutdown property
    #[dbus_proxy(property)]
    fn preparing_for_shutdown(&self) -> zbus::Result<bool>;

    /// PreparingForSleep property
    #[dbus_proxy(property)]
    fn preparing_for_sleep(&self) -> zbus::Result<bool>;

    /// RebootParameter property
    #[dbus_proxy(property)]
    fn reboot_parameter(&self) -> zbus::Result<String>;

    /// RebootToBootLoaderEntry property
    #[dbus_proxy(property)]
    fn reboot_to_boot_loader_entry(&self) -> zbus::Result<String>;

    /// RebootToBootLoaderMenu property
    #[dbus_proxy(property)]
    fn reboot_to_boot_loader_menu(&self) -> zbus::Result<u64>;

    /// RebootToFirmwareSetup property
    #[dbus_proxy(property)]
    fn reboot_to_firmware_setup(&self) -> zbus::Result<bool>;

    /// RemoveIPC property
    #[dbus_proxy(property)]
    fn remove_ipc(&self) -> zbus::Result<bool>;

    /// RuntimeDirectoryInodesMax property
    #[dbus_proxy(property)]
    fn runtime_directory_inodes_max(&self) -> zbus::Result<u64>;

    /// RuntimeDirectorySize property
    #[dbus_proxy(property)]
    fn runtime_directory_size(&self) -> zbus::Result<u64>;

    /// ScheduledShutdown property
    #[dbus_proxy(property)]
    fn scheduled_shutdown(&self) -> zbus::Result<(String, u64)>;

    /// SessionsMax property
    #[dbus_proxy(property)]
    fn sessions_max(&self) -> zbus::Result<u64>;

    /// UserStopDelayUSec property
    #[dbus_proxy(property)]
    fn user_stop_delay_usec(&self) -> zbus::Result<u64>;

    /// WallMessage property
    #[dbus_proxy(property)]
    fn wall_message(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_wall_message(&self, value: &str) -> zbus::Result<()>;
}
