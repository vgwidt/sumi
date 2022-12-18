use std::fmt;
use std::ops::Deref;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;
use crate::types::{MyUser, UserInfo};

pub struct UseUserContextHandle {
    inner: UseStateHandle<MyUser>,
    history: Navigator,
}

impl UseUserContextHandle {
    pub fn login(&self, value: MyUser) {
        self.inner.set(value);
        self.history.push(&AppRoute::Home);
    }

    pub fn ctx_logout(&self) {
        self.inner.set(MyUser::default());
        self.history.push(&AppRoute::Home);
    }

    pub fn update_info(&self, userinfo: UserInfo) {
        let mut user = self.inner.deref().clone();

        user.username = userinfo.username;
        user.display_name = userinfo.display_name;
        user.email = userinfo.email;

        self.inner.set(user);
    }
}

impl Deref for UseUserContextHandle {
    type Target = MyUser;

    fn deref(&self) -> &Self::Target {
        &(*self.inner)
    }
}

impl Clone for UseUserContextHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            history: self.history.clone(),
        }
    }
}

impl PartialEq for UseUserContextHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

impl fmt::Debug for UseUserContextHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseUserContextHandle")
            .field("value", &format!("{:?}", *self.inner))
            .finish()
    }
}

#[hook]
pub fn use_user_context() -> UseUserContextHandle {
    let inner = use_context::<UseStateHandle<MyUser>>().unwrap();
    let history = use_navigator().unwrap();

    UseUserContextHandle { inner, history }
}
