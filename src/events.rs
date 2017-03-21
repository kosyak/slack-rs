//
// Copyright 2015-2016 the slack-rs authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use api::{Bot, Message, File, FileComment, Channel, User, MessageUnpinnedItem, MessagePinnedItem,
          stars, reactions};
use std::boxed::Box;

/// Represents Slack [rtm event](https://api.slack.com/rtm) types.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Event {
    /// Represents the slack [`hello`](https://api.slack.com/events/hello) event.
    Hello,
    /// Represents the slack [`message`](https://api.slack.com/events/message)
    /// event.
    Message(Box<Message>),
    /// Represents the slack
    /// [`user_typing`](https://api.slack.com/events/user_typing) event.
    UserTyping { channel: String, user: String },
    /// Represents the slack
    /// [`channel_marked`](https://api.slack.com/events/channel_marked) event.
    ChannelMarked { channel: String, ts: String },
    /// Represents the slack
    /// [`channel_created`](https://api.slack.com/events/channel_created) event.
    ChannelCreated { channel: Box<Channel> },
    /// Represents the slack
    /// [`channel_joined`](https://api.slack.com/events/channel_joined) event.
    ChannelJoined { channel: Box<Channel> },
    /// Represents the slack
    /// [`channel_left`](https://api.slack.com/events/channel_left) event.
    ChannelLeft { channel: String },
    /// Represents the slack
    /// [`channel_deleted`](https://api.slack.com/events/channel_deleted) event.
    ChannelDeleted { channel: String },
    /// Represents the slack
    /// [`channel_rename`](https://api.slack.com/events/channel_rename) event.
    ChannelRename { channel: Box<Channel> },
    /// Represents the slack
    /// [`channel_archive`](https://api.slack.com/events/channel_archive) event.
    ChannelArchive { channel: String, user: String },
    /// Represents the slack
    /// [`channel_unarchive`](https://api.slack.com/events/channel_unarchive) event.
    ChannelUnArchive { channel: String, user: String },
    /// Represents the slack
    /// [`channel_history_changed`](https://api.slack.
    /// com/events/channel_history_changed) event.
    ChannelHistoryChanged {
        latest: String,
        ts: String,
        event_ts: String,
    },
    /// Represents the slack
    /// [`im_created`](https://api.slack.com/events/im_created) event.
    ImCreated { user: String, channel: Box<Channel> },
    /// Represents the slack [`im_open`](https://api.slack.com/events/im_open)
    /// event.
    ImOpen { user: String, channel: String },
    /// Represents the slack [`im_close`](https://api.slack.com/events/im_close)
    /// event.
    ImClose { user: String, channel: String },
    /// Represents the slack [`im_marked`](https://api.slack.com/events/im_marked)
    /// event.
    ImMarked { channel: String, ts: String },
    /// Represents the slack
    /// [`im_history_changed`](https://api.slack.com/events/im_history_changed)
    /// event.
    ImHistoryChanged {
        latest: String,
        ts: String,
        event_ts: String,
    },
    /// Represents the slack
    /// [`group_joined`](https://api.slack.com/events/group_joined) event.
    GroupJoined { channel: Box<Channel> },
    /// Represents the slack
    /// [`group_left`](https://api.slack.com/events/group_left) event.
    GroupLeft { channel: Box<Channel> },
    /// Represents the slack
    /// [`group_open`](https://api.slack.com/events/group_open) event.
    GroupOpen { user: String, channel: String },
    /// Represents the slack
    /// [`group_close`](https://api.slack.com/events/group_close) event.
    GroupClose { user: String, channel: String },
    /// Represents the slack
    /// [`group_archive`](https://api.slack.com/events/group_archive) event.
    GroupArchive { channel: String },
    /// Represents the slack
    /// [`group_unarchive`](https://api.slack.com/events/group_unarchive) event.
    GroupUnArchive { channel: String },
    /// Represents the slack
    /// [`group_rename`](https://api.slack.com/events/group_rename) event.
    GroupRename { channel: Box<Channel> },
    /// Represents the slack
    /// [`group_marked`](https://api.slack.com/events/group_marked) event.
    GroupMarked { channel: String, ts: String },
    /// Represents the slack
    /// [`group_history_changed`](https://api.slack.
    /// com/events/group_history_changed) event.
    GroupHistoryChanged {
        latest: String,
        ts: String,
        event_ts: String,
    },
    /// Represents the slack
    /// [`file_created`](https://api.slack.com/events/file_created) event.
    FileCreated { file: Box<File> },
    /// Represents the slack
    /// [`file_shared`](https://api.slack.com/events/file_shared) event.
    FileShared { file: Box<File> },
    /// Represents the slack
    /// [`file_unshared`](https://api.slack.com/events/file_unshared) event.
    FileUnShared { file: Box<File> },
    /// Represents the slack
    /// [`file_public`](https://api.slack.com/events/file_public) event.
    FilePublic { file: Box<File> },
    /// Represents the slack
    /// [`file_private`](https://api.slack.com/events/file_private) event.
    FilePrivate { file: String },
    /// Represents the slack
    /// [`file_change`](https://api.slack.com/events/file_change) event.
    FileChange { file: Box<File> },
    /// Represents the slack
    /// [`file_deleted`](https://api.slack.com/events/file_deleted) event.
    FileDeleted { file_id: String, event_ts: String },
    /// Represents the slack
    /// [`file_comment_added`](https://api.slack.com/events/file_comment_added)
    /// event.
    FileCommentAdded {
        file: Box<File>,
        comment: FileComment,
    },
    /// Represents the slack
    /// [`file_comment_edited`](https://api.slack.com/events/file_comment_edited)
    /// event.
    FileCommentEdited {
        file: Box<File>,
        comment: FileComment,
    },
    /// Represents the slack
    /// [`file_comment_deleted`](https://api.slack.com/events/file_comment_deleted)
    /// event.
    FileCommentDeleted { file: Box<File>, comment: String },
    /// Represents the slack [`pin_added`](https://api.slack.com/events/pin_added)
    /// event.
    PinAdded {
        user: String,
        channel_id: String,
        item: Box<MessagePinnedItem>,
        event_ts: String,
    },
    /// Represents the slack
    /// [`pin_removed`](https://api.slack.com/events/pin_removed) event.
    PinRemoved {
        user: String,
        channel_id: String,
        item: Box<MessageUnpinnedItem>,
        has_pins: bool,
        event_ts: String,
    },
    /// Represents the slack
    /// [`presence_change`](https://api.slack.com/events/presence_change) event.
    PresenceChange { user: String, presence: String },
    /// Represents the slack
    /// [`manual_presence_change`](https://api.slack.
    /// com/events/manual_presence_change) event.
    ManualPresenceChange { presence: String },
    /// Represents the slack
    /// [`pref_change`](https://api.slack.com/events/pref_change) event.
    PrefChange { name: String, value: String },
    /// Represents the slack
    /// [`user_change`](https://api.slack.com/events/user_change) event.
    UserChange { user: Box<User> },
    /// Represents the slack [`team_join`](https://api.slack.com/events/team_join)
    /// event.
    TeamJoin { user: Box<User> },
    /// Represents the slack
    /// [`star_added`](https://api.slack.com/events/star_added) event.
    StarAdded {
        user: String,
        item: Box<stars::ListResponseItem>,
        event_ts: String,
    },
    /// Represents the slack
    /// [`star_removed`](https://api.slack.com/events/star_removed) event.
    StarRemoved {
        user: String,
        item: Box<stars::ListResponseItem>,
        event_ts: String,
    },
    /// Represents the slack
    /// [`reaction_added`](https://api.slack.com/events/reaction_added) event.
    ReactionAdded {
        user: String,
        reaction: String,
        item: Box<reactions::ListResponseItem>,
        item_user: String,
        event_ts: String,
    },
    /// Represents the slack
    /// [`reaction_removed`](https://api.slack.com/events/reaction_removed) event.
    ReactionRemoved {
        user: String,
        reaction: String,
        item: Box<reactions::ListResponseItem>,
        item_user: String,
        event_ts: String,
    },
    /// Represents the slack
    /// [`emoji_changed`](https://api.slack.com/event/emoji_changed) event.
    EmojiChanged { event_ts: String },
    /// Represents the slack
    /// [`commands_changed`](https://api.slack.com/event/commands_changed) event.
    CommandsChanged { event_ts: String },
    /// Represents the slack
    /// [`team_plan_change`](https://api.slack.com/event/team_plan_change) event.
    TeamPlanChange { plan: String },
    /// Represents the slack
    /// [`team_pref_change`](https://api.slack.com/event/team_pref_change) event.
    TeamPrefChange { name: String, value: bool },
    /// Represents the slack
    /// [`team_rename`](https://api.slack.com/event/team_rename) event.
    TeamRename { name: String },
    /// Represents the slack
    /// [`team_domain_change`](https://api.slack.com/event/team_domain_change)
    /// event.
    TeamDomainChange { url: String, domain: String },
    /// Represents the slack
    /// [`email_domain_changeed`](https://api.slack.
    /// com/event/email_domain_changeed) event.
    EmailDomainChanged {
        email_domain: String,
        event_ts: String,
    },
    /// Represents the slack [`bot_added`](https://api.slack.com/event/bot_added)
    /// event.
    BotAdded { bot: Bot },
    /// Represents the slack
    /// [`bot_changed`](https://api.slack.com/event/bot_changed) event.
    BotChanged { bot: Bot },
    /// Represents the slack
    /// [`accounts_changed`](https://api.slack.com/event/accounts_changed) event.
    AccountsChanged,
    /// Represents the slack
    /// [`team_migration_started`](https://api.slack.
    /// com/event/team_migration_started) event.
    TeamMigrationStarted,
    /// Represents the slack
    /// [`reconnect_url`](https://api.slack.com/event/reconnect_url)
    /// event.
    ReconnectUrl,
    /// Represents a confirmation of a message sent
    MessageSent {
        reply_to: isize,
        ts: String,
        text: String,
    },
    /// Represents an error sending a message
    MessageError {
        reply_to: isize,
        code: isize,
        message: String,
    },
}
