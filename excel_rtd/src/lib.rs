extern crate com_rs;

//GUID = f7ff4a68-5147-4204-a0f8-1a19580eb6c2
//ProgId = "RtdServer.SimpleRtdServer

com_interface! {
    interface IRTDServer: IUnknown {
        _callback: IRTDUpdateEvent,
        _timer: Timer,
        _topicId: u32,
        fn SimpleRTDServer() -> interface;
        fn ServerStart() -> u32;
        fn ServerTerminate() -> ();
        fn Heartbeat() -> u32;
        fn ConnectData() -> IUnknown;
        fn DisconnectData() -> ();
        fn TimerEventHandler() -> ();
        fn RefreshData() -> Vec;
        fn GetTime() -> String;
    }
}

