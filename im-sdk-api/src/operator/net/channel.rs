pub static NET_CHANNEL: once_cell::sync::Lazy<once_cell::sync::OnceCell<NetChannel>> =
    once_cell::sync::Lazy::new(once_cell::sync::OnceCell::new);

pub(crate) fn net_channel_generator<'a>() -> &'a NetChannel {
    NET_CHANNEL.get_or_init(|| {
        let (sender, join) = handle();
        std::thread::spawn(move || {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(join);
        });
        sender
    })
}

pub(crate) struct NetChannel(tokio::sync::mpsc::UnboundedSender<Event>);

impl NetChannel {
    pub(crate) fn send(&self, message: Event) -> Result<(), crate::ChannelError> {
        self.0
            .send(message)
            .map_err(|_| crate::ChannelError::SendFailed)
    }
}

pub(crate) enum Event {
    Join {
        from_id: u32,
        recv_id: u32,
        sender: super::codec::CodecChannel,
    },
    // #[cfg(not(feature = "mock"))]
    Send {
        from_id: u32,
        recv_list: Vec<u32>,
        data: im_net::Packet,
    },
}

struct ChatList {
    list: std::collections::HashMap<u32, super::codec::CodecChannel>,
}

impl ChatList {
    fn insert(&mut self, recv_id: u32, sender: &super::codec::CodecChannel) {
        if let Some(s) = self.list.get_mut(&recv_id) {
            *s = sender.to_owned();
        } else {
            self.list.insert(recv_id, sender.to_owned());
        }
    }
}

type UserList = std::collections::HashMap<u32, ChatList>;

pub fn handle() -> (NetChannel, impl std::future::Future<Output = ()>) {
    use tokio_stream::StreamExt as _;
    let (sender, sender_rx) = tokio::sync::mpsc::unbounded_channel::<Event>();
    let mut collect_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(sender_rx);
    let join_handle = async move {
        let mut user_list = UserList::new();
        while let Some(event) = collect_rx.next().await {
            match event {
                Event::Join {
                    from_id,
                    recv_id,
                    sender,
                } => {
                    user_list
                        .entry(from_id)
                        .and_modify(|list: &mut ChatList| {
                            list.insert(recv_id, &sender);
                        })
                        .or_insert(ChatList {
                            list: std::collections::HashMap::from([(recv_id, sender)]),
                        });
                }
                // #[cfg(not(feature = "mock"))]
                Event::Send {
                    from_id,
                    recv_list,
                    data,
                } => {
                    if let Some(tx) = user_list.get(&from_id) {
                        for recv in recv_list {
                            if let Some(list) = user_list.get(&from_id)
                                && let Some(tx) = list.list.get(&recv)
                            {
                                if let Err(e) = tx.send(data.clone()) {
                                    tracing::error!("[codec channel handle] send error: {e}");
                                };
                            }
                        }
                    }
                }
            }
        }
    };
    (NetChannel(sender), join_handle)
}
