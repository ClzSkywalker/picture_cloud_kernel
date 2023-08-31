use super::eventx::{BaseDomainEvent, IDomainType};

pub trait IDomainEventPublisher {
    type Data;
    type E: IDomainType;
    fn publish(event:BaseDomainEvent<Self::Data,Self::E>);
    fn publish_and_save(event:BaseDomainEvent<Self::Data,Self::E>);
}