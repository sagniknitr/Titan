pub struct Node {
    handle: Rc<NodeHandle>,
    pub(crate) context: Rc<ContextHandle>,
    pub(crate) subscriptions: Vec<Weak<SubscriptionBase>>,
}