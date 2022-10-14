use std::collections::BTreeMap;

use lazy_static::lazy_static;

lazy_static! {
    /// name -> (upper bank, patch)
    pub static ref INSTRUMENTS_BY_NAME: BTreeMap<&'static str, (u8, u8)> = {
        let mut m = BTreeMap::new();
        /*
        m.insert("Synth Bass 1", (0, 0));
        m.insert("Air Blow", (0, 1));
        m.insert("Whistle Slap", (0, 2));
        m.insert("Synth Pluck", (0, 3));
        */

        m.insert("Marimba", (3, 0x00));
        m.insert("Marimba 2", (3, 0x01));
        m.insert("Marimba 3", (3, 0x02));
        m.insert("Xylophone 1", (3, 0x03));
        m.insert("Xylophone 2", (3, 0x04));
        m.insert("Xylophone 3", (3, 0x05));
        m.insert("Vibraphone 1", (3, 0x06));
        m.insert("Vibraphone 2", (3, 0x07));
        m.insert("Vibraphone 3", (3, 0x08));
        m.insert("Celesta 1", (3, 0x09));
        m.insert("Celesta 2", (3, 0x0A));
        m.insert("Huff n' Puff Synth [Lead 4 (chiff)] 1", (3, 0x0B));
        m.insert("Huff n' Puff Synth [Lead 4 (chiff)] 2", (3, 0x0C));
        m.insert("Huff n' Puff Synth [Lead 4 (chiff)] 3", (3, 0x0D));
        m.insert("Old School Piano", (3, 0x0E));
        m.insert("Xylophone 4", (3, 0x0F));
        m.insert("Cello", (3, 0x10));
        m.insert("Viola", (3, 0x11));
        m.insert("Violin", (3, 0x12));
        m.insert("Violin 2", (3, 0x13));
        m.insert("Pizzicato Strings 1A", (3, 0x14));
        m.insert("Pizzicato Strings 1B", (3, 0x15));
        m.insert("Pizzicato Strings 2A", (3, 0x16));
        m.insert("Pizzicato Strings 2B", (3, 0x17));
        m.insert("String Ensemble", (3, 0x18));
        m.insert("Synth String 1", (3, 0x19));
        m.insert("Synth String 2", (3, 0x1A));
        m.insert("Synth Flute (?)", (3, 0x1B));
        m.insert("Timpani 1A", (3, 0x1C));
        m.insert("Timpani 1B", (3, 0x1D));
        m.insert("Timpani 2A", (3, 0x1E));
        m.insert("Timpani 2B", (3, 0x1F));
        m.insert("Electric Piano 1A", (3, 0x20));
        m.insert("Electric Piano 1B", (3, 0x21));
        m.insert("Electric Piano 2", (3, 0x22));
        m.insert("Acoustic Piano 1", (3, 0x23));
        m.insert("Acoustic Piano 2", (3, 0x24));
        m.insert("Music Box 1", (3, 0x25));
        m.insert("Music Box 2", (3, 0x26));
        m.insert("Nylon Guitar 1", (3, 0x27));
        m.insert("Nylon Guitar 2", (3, 0x28));
        m.insert("Acoustic Guitar 1A", (3, 0x29));
        m.insert("Acoustic Guitar 1B", (3, 0x2A));
        m.insert("Acoustic Guitar 2A", (3, 0x2B));
        m.insert("Acoustic Guitar 2B", (3, 0x2C));
        m.insert("English Horn 1", (3, 0x2D));
        m.insert("English Horn 2", (3, 0x2E));
        m.insert("Synth Bass 1", (3, 0x2F));
        m.insert("French Horn", (3, 0x30));
        m.insert("Tuba 1", (3, 0x31));
        m.insert("Tuba 2", (3, 0x32));
        m.insert("Trombone 1", (3, 0x33));
        m.insert("Trombone 2", (3, 0x34));
        m.insert("Bassoon 1", (3, 0x35));
        m.insert("Bassoon 2", (3, 0x36));
        m.insert("Bassoon 3", (3, 0x37));
        m.insert("Clarinet", (3, 0x38));
        m.insert("Alto Sax", (3, 0x39));
        m.insert("Oboe 1A", (3, 0x3A));
        m.insert("Oboe 1B", (3, 0x3B));
        m.insert("Oboe 2", (3, 0x3C));
        m.insert("Muted Flute", (3, 0x3D));
        m.insert("Koopa Bros Synth A", (3, 0x3E));
        m.insert("Koopa Bros Synth B", (3, 0x3F));
        m.insert("Acoustic Bass 1", (3, 0x40));
        m.insert("Acoustic Bass 2", (3, 0x41));
        m.insert("Synth Brass 1", (3, 0x42));
        m.insert("Synth Brass 2", (3, 0x43));
        m.insert("Overdriven Guitar 1", (3, 0x44));
        m.insert("Overdriven Guitar 2", (3, 0x45));
        m.insert("Kalimba", (3, 0x46));
        m.insert("Flute 1", (3, 0x47));
        m.insert("Flute 2", (3, 0x48));
        m.insert("Flute 3", (3, 0x49));
        m.insert("Steel Drum 1", (3, 0x4A));
        m.insert("Steel Drum 2", (3, 0x4B));
        m.insert("Steel Drum 3", (3, 0x4C));
        m.insert("Percussive Organ 1", (3, 0x4D));
        m.insert("Drawbar Organ 1", (3, 0x4E));
        m.insert("Drawbar Organ 2", (3, 0x4F));
        m.insert("Muted Trumpet 1", (3, 0x50));
        m.insert("Muted Trumpet 2", (3, 0x51));
        m.insert("Guitar Harmonics 1", (3, 0x52));
        m.insert("Guitar Harmonics 2", (3, 0x53));
        m.insert("Percussive Organ 2", (3, 0x54));
        m.insert("Sitar 1", (3, 0x55));
        m.insert("Bari Sax 1", (3, 0x56));
        m.insert("Bari Sax 2", (3, 0x57));
        m.insert("Muted Trumpet 3", (3, 0x58));
        m.insert("Choir 1A [Lead 6 (voice)]", (3, 0x59));
        m.insert("Choir 1B [Lead 6 (voice)]", (3, 0x5A));
        m.insert("Choir 2", (3, 0x5B));
        m.insert("Electric Bass 1", (3, 0x5C));
        m.insert("Electric Bass 2", (3, 0x5D));
        m.insert("Vibraphone 4", (3, 0x5E));
        m.insert("Vibraphone 5", (3, 0x5F));
        m.insert("Harpsichord 1", (3, 0x60));
        m.insert("Harpsichord 2", (3, 0x61));
        m.insert("Harpsichord 3", (3, 0x62));
        m.insert("Rock Organ 1", (3, 0x63));
        m.insert("Rock Organ 2", (3, 0x64));
        m.insert("Muted Synth Bass", (3, 0x65));
        m.insert("Sitar 2", (3, 0x66));
        m.insert("Sitar 3", (3, 0x67));
        m.insert("Synth Bass 2", (3, 0x68));
        m.insert("Synth Bass 3", (3, 0x69));
        m.insert("Synth Brass 3", (3, 0x6A));
        m.insert("Synth Brass 4", (3, 0x6B));
        m.insert("Whistle 1", (3, 0x6C));
        m.insert("Whistle 2", (3, 0x6D));
        m.insert("Blown Bottle 1", (3, 0x6E));
        m.insert("Blown Bottle 2", (3, 0x6F));
        m.insert("Shooting Star Pad", (3, 0x70));
        m.insert("Slap Bass", (3, 0x71));
        m.insert("Tubular Bells", (3, 0x72));
        m.insert("Sawtooth Synth", (3, 0x73));
        m.insert("Synth(?)", (3, 0x74));
        m.insert("Electric Piano 3", (3, 0x75));
        m.insert("Electric Piano 4", (3, 0x76));
        m.insert("Electric Piano 5", (3, 0x77));
        m.insert("Electric Piano 6", (3, 0x78));
        m.insert("Music Box 3", (3, 0x79));
        m.insert("Vibraphone 6", (3, 0x7A));
        m.insert("Celesta 3", (3, 0x7B));
        m.insert("Plucked Electric Bass", (3, 0x7C));
        m.insert("Whistle 3", (3, 0x7D));
        m.insert("Harmonized Synth Voice 1", (3, 0x7E));
        m.insert("Harmonized Synth Voice 2", (3, 0x7F));
        m.insert("Glockenspiel 1", (3, 0x80));
        m.insert("Glockenspiel 2", (3, 0x81));
        m.insert("Dulcimer 1", (3, 0x82));
        m.insert("Dulcimer 2", (3, 0x83));
        m.insert("Beep", (3, 0x84));
        m.insert("Clean Beep", (3, 0x85));
        m.insert("Sitar 4", (3, 0x86));
        m.insert("Kalimba 2", (3, 0x87));
        m.insert("Orchestra Hit", (3, 0x88));
        m.insert("Accordion 1", (3, 0x89));
        m.insert("Accordion 2", (3, 0x8A));
        m.insert("Accordion 3", (3, 0x8B));
        m.insert("Vibraphone 7", (3, 0x8C));
        m.insert("Distortion Strings", (3, 0x8D));
        m.insert("Music Box 4", (3, 0x8E));
        m.insert("Music Box 5", (3, 0x8F));
        m.insert("Harp", (3, 0x90));
        m.insert("Harsh Piano", (3, 0x91));
        m.insert("Harpsichord 4", (3, 0x92));
        m.insert("Harpsichord 5", (3, 0x93));
        m.insert("Tenor Sax 1", (3, 0x94));
        m.insert("Tenor Sax 2", (3, 0x95));
        m.insert("Synth Bass 4", (3, 0x96));
        m.insert("Synth Bass 5", (3, 0x97));
        m.insert("Mosquito", (3, 0x98));
        m.insert("Cat [Lead 8 (bass + lead)]", (3, 0x99));
        m.insert("Music Box 6", (3, 0xA0));
        m.insert("Music Box 7", (3, 0xA1));
        m.insert("Plucked Percussion (?)", (3, 0xA2));
        m.insert("Church Organ", (3, 0xA3));
        m.insert("Reverse Cymbal", (3, 0xA4));
        m.insert("Synth Voice", (3, 0xA5));

        m.insert("Woodblock", (3, 0xE4));

        m
    };

    pub static ref INSTRUMENTS_BY_ID: BTreeMap<(u8, u8), &'static str> = {
        let mut m = BTreeMap::new();
        for (k, v) in INSTRUMENTS_BY_NAME.iter() {
            m.insert(*v, *k);
        }
        m
    };
}
