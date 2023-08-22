use std::fmt::Debug;

use crate::json::json::my_date_format_local;
use chrono::{DateTime, Local};
use serde::Serialize;

pub trait IDomainEvent: Serialize + Clone{
    /// A version of the `event_type`, used for event upcasting.
    fn event_version(&self) -> String;
    /// 修改事件状态为成功
    fn handle_success(&mut self);
    /// 修改事件状态为失败
    fn handle_failed(&mut self);
}

/// 事件类型
pub trait IDomainType {
    fn event_type(&self) -> String;
}

/// 事件状态
#[derive(Debug, Serialize, Clone)]
pub enum EventStatusEnum {
    Pending,
    Success,
    Failure,
}

/// 领域事件基类
#[derive(Debug, Clone, Serialize)]
pub struct BaseDomainEvent<T, E>
where
    E: IDomainType,
{
    // 幂等键:即为当前事件的id
    pub id: String,
    // 领域对象id
    pub domain_id: String,
    // 事件状态
    pub event_status: EventStatusEnum,
    /// 事件类型
    pub event_type: E,
    /// 业务发生时间
    #[serde(with = "my_date_format_local")]
    pub occurred_on: DateTime<Local>,
    /// 领域事件数据
    pub data: T,
}

impl<T, E> IDomainEvent for BaseDomainEvent<T, E>
where
    T: Send + Sync + Clone + Serialize,
    E: IDomainType + Send + Sync + Clone + Serialize,
{
    fn event_version(&self) -> String {
        todo!()
    }

    fn handle_success(&mut self) {
        self.event_status = EventStatusEnum::Success;
    }

    fn handle_failed(&mut self) {
        self.event_status = EventStatusEnum::Failure
    }
}
