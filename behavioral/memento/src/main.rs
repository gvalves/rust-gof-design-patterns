use memento::{Image, ImageBackupManager, ImageFormat};

pub fn main() {
    let image = Image::new("/media/image.png", ImageFormat::Png).into();
    let mut backup_manager = ImageBackupManager::new(&image);

    backup_manager.backup();
    image.borrow_mut().convert_format_to(ImageFormat::Jpg);

    backup_manager.backup();
    image.borrow_mut().convert_format_to(ImageFormat::Gif);

    backup_manager.undo();
    backup_manager.undo();
    backup_manager.undo();

    println!("{:?}", image);
}
