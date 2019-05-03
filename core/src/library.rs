use crate::character::Character;
use crate::display_object::DisplayObject;
use crate::graphic::Graphic;
use crate::movie_clip::MovieClip;
use std::collections::HashMap;
use swf::CharacterId;

pub struct Library {
    characters: HashMap<CharacterId, Character>,
    jpeg_tables: Option<Vec<u8>>,
}

impl Library {
    pub fn new() -> Library {
        Library {
            characters: HashMap::new(),
            jpeg_tables: None,
        }
    }

    pub fn register_character(&mut self, id: CharacterId, character: Character) {
        // TODO(Herschel): What is the behavior if id already exists?
        self.characters.insert(id, character);
    }

    pub fn contains_character(&self, id: CharacterId) -> bool {
        self.characters.contains_key(&id)
    }

    pub fn instantiate_display_object(
        &self,
        id: CharacterId,
    ) -> Result<DisplayObject, Box<std::error::Error>> {
        match self.characters.get(&id) {
            Some(Character::Graphic {
                x_min,
                y_min,
                shape_handle,
            }) => Ok(DisplayObject::new(Box::new(Graphic::new(
                *shape_handle,
                *x_min,
                *y_min,
            )))),
            Some(Character::MovieClip {
                tag_stream_start,
                num_frames,
            }) => Ok(DisplayObject::new(Box::new(MovieClip::new_with_data(
                *tag_stream_start,
                *num_frames,
            )))),
            Some(_) => Err("Not a DisplayObject".into()),
            None => Err("Character id doesn't exist".into()),
        }
    }

    pub fn set_jpeg_tables(&mut self, data: Vec<u8>) {
        self.jpeg_tables = Some(data);
    }

    pub fn jpeg_tables(&self) -> Option<&[u8]> {
        self.jpeg_tables.as_ref().map(|data| &data[..])
    }
}
