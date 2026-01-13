use iced::Task;
use iced::widget::text_editor;
use crate::{InventoryApp, Message};
use crate::note::Note;
use crate::audit::{AuditAction, AuditEntry};

impl InventoryApp {
    pub fn handle_create_note(&mut self) -> Task<Message> {
        let new_note = Note::new("Untitled Note".to_string());
        self.selected_note_id = Some(new_note.id.clone());
        self.note_title_input = new_note.title.clone();
        self.editor_content = text_editor::Content::new();
        
        // Log note creation
        if let Some(session) = &self.session {
            let audit_entry = AuditEntry::new(
                session.user_id.clone(),
                session.username.clone(),
                AuditAction::NoteCreated,
                "note".to_string(),
                Some(new_note.id.clone()),
                format!("Created note: {}", new_note.title),
            );
            self.audit_log.add_entry(audit_entry);
        }
        
        self.notes.push(new_note);
        self.auto_save()
    }

    pub fn handle_select_note(&mut self, note_id: String) {
        if let Some(note) = self.notes.iter().find(|n| n.id == note_id) {
            self.selected_note_id = Some(note.id.clone());
            self.note_title_input = note.title.clone();
            self.editor_content = text_editor::Content::with_text(&note.content);
        }
    }

    pub fn handle_update_note_title(&mut self, title: String) -> Task<Message> {
        self.note_title_input = title.clone();
        if let Some(note_id) = &self.selected_note_id {
            if let Some(note) = self.notes.iter_mut().find(|n| n.id == *note_id) {
                note.update_title(title);
                return self.auto_save();
            }
        }
        Task::none()
    }

    pub fn handle_update_note_content(&mut self, action: text_editor::Action) -> Task<Message> {
        self.editor_content.perform(action);
        if let Some(note_id) = &self.selected_note_id {
            if let Some(note) = self.notes.iter_mut().find(|n| n.id == *note_id) {
                note.update_content(self.editor_content.text());
                return self.auto_save();
            }
        }
        Task::none()
    }

    pub fn handle_delete_note(&mut self, note_id: String) {
        self.delete_note_confirm = Some(note_id);
    }

    pub fn handle_confirm_delete_note(&mut self) -> Task<Message> {
        if let Some(note_id) = &self.delete_note_confirm {
            let deleted_note = self
                .notes
                .iter()
                .find(|n| n.id == *note_id)
                .map(|n| n.title.clone());
            
            self.notes.retain(|note| note.id != *note_id);

            // Log note deletion
            if let Some(session) = &self.session {
                if let Some(note_title) = deleted_note {
                    let audit_entry = AuditEntry::new(
                        session.user_id.clone(),
                        session.username.clone(),
                        AuditAction::NoteDeleted,
                        "note".to_string(),
                        Some(note_id.clone()),
                        format!("Deleted note: {}", note_title),
                    );
                    self.audit_log.add_entry(audit_entry);
                }
            }

            // If deleted note was selected, select another or clear
            if self.selected_note_id.as_ref() == Some(note_id) {
                if let Some(first_note) = self.notes.first() {
                    self.selected_note_id = Some(first_note.id.clone());
                    self.note_title_input = first_note.title.clone();
                    self.editor_content = text_editor::Content::with_text(&first_note.content);
                } else {
                    self.selected_note_id = None;
                    self.note_title_input.clear();
                    self.editor_content = text_editor::Content::new();
                }
            }

            self.delete_note_confirm = None;
            return self.auto_save();
        }
        Task::none()
    }

    pub fn handle_close_delete_confirm(&mut self) {
        self.delete_note_confirm = None;
    }
}

