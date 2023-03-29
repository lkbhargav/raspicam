/// Settings for the camera.
///
/// # Example
///
/// ```
/// use raspicam::image::camera_operations::click_image;
/// use raspicam::image::settings::CameraSettings;
///
/// let camera_settings: CameraSettings = CameraSettings {
///        sharpness: "50",//or any value you want to modify
///        ..Default::default()
///    };
/// let test_camera_settings: CameraSettings = CameraSettings {
///           contrast: "50",
///           sharpness: "50",
///           brightness: "60",
///           saturation: "0",
///           quality: "100",
///           timeout: "3000",
///           iso: "300",
///           output: "~/raspicam.jpg",
/// };
///
/// assert_eq!(camera_settings, test_camera_settings);
/// ```
///
#[derive(Debug, PartialEq, Clone)]
pub struct CameraSettings {
    pub contrast: String,
    pub sharpness: String,
    pub brightness: String,
    pub saturation: String,
    pub quality: String,
    pub timeout: String,
    pub iso: String,
    pub output: String,
}

impl Default for CameraSettings {
    /// Initialize CameraSettings with the default values
    ///
    /// # Example
    ///
    /// ```
    /// use raspicam::image::settings::CameraSettings;
    ///
    /// let camera_settings: CameraSettings = CameraSettings::default();
    /// let test_camera_settings: CameraSettings = CameraSettings {
    ///           contrast: "50",
    ///           sharpness: "30",
    ///           brightness: "60",
    ///           saturation: "0",
    ///           quality: "100",
    ///           timeout: "3000",
    ///           iso: "300",
    ///           output: "~/raspicam.jpg",
    /// };
    ///
    /// assert_eq!(camera_settings, test_camera_settings);
    /// ```
    ///
    fn default() -> CameraSettings {
        CameraSettings {
            contrast: "50".to_string(),
            sharpness: "30".to_string(),
            brightness: "60".to_string(),
            saturation: "0".to_string(),
            quality: "100".to_string(),
            timeout: "3000".to_string(),
            iso: "300".to_string(),
            output: "~/raspicam.jpg".to_string(),
        }
    }
}

impl CameraSettings {
    pub fn set_output(&mut self, output: String) {
        self.output = output;
    }
}

/// Settings for the image.
///
/// # Example
///
/// ```
/// use raspicam::image::camera_operations::click_image;
/// use raspicam::image::settings::ImageSettings;
///
/// let image_settings: ImageSettings = ImageSettings {
///        width: "50",//or any value you want to modify
///        ..Default::default()
///    };
/// let test_image_settings: ImageSettings = ImageSettings {
///         width: "50",
///         height: "200",
///         rotation: "180",
///         horizontal_flip: "false",
///         vertical_flip: "false",
/// };
///
/// assert_eq!(image_settings, test_image_settings);
/// ```
///
#[derive(Debug, PartialEq, Clone)]
pub struct ImageSettings {
    pub width: String,
    pub height: String,
    pub rotation: String,
    pub horizontal_flip: String,
    pub vertical_flip: String,
}

impl Default for ImageSettings {
    /// Initialize ImageSettings with the default values
    ///
    /// # Example
    ///
    /// ```
    /// use raspicam::image::settings::ImageSettings;
    ///
    /// let image_settings: ImageSettings = ImageSettings::default();
    /// let test_image_settings: ImageSettings = ImageSettings {
    ///         width: "200",
    ///         height: "200",
    ///         rotation: "180",
    ///         horizontal_flip: "false",
    ///         vertical_flip: "false",
    /// };
    ///
    /// assert_eq!(image_settings, test_image_settings);
    /// ```
    ///
    fn default() -> ImageSettings {
        ImageSettings {
            width: "200".to_string(),
            height: "200".to_string(),
            rotation: "180".to_string(),
            horizontal_flip: "false".to_string(),
            vertical_flip: "false".to_string(),
        }
    }
}
