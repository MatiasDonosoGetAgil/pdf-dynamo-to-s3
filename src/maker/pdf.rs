use chrono::DateTime;
use printpdf::path::{PaintMode, WindingOrder};
use printpdf::*;
use std::collections::HashMap;
use std::io::Cursor;
use std::vec;
use ttf_parser::Face;

// const PAGE_WIDTH: f32 = 80.0;
const DPI: f32 = 300.0;

// Definimos los bytes de las fuentes directamente
const FONT_BOLD: &[u8] = include_bytes!("../../assets/fonts/segoe-ui-bold.ttf");
const FONT_LIGHT: &[u8] = include_bytes!("../../assets/fonts/segoe-ui.ttf");

// Definimos los bytes de las imágenes directamente
const ICON_CAMINO: &[u8] = include_bytes!("../../assets/img/camino.bmp");
const ICON_CUBIERTOS: &[u8] = include_bytes!("../../assets/img/cubiertos.bmp");
const ICON_DINERO: &[u8] = include_bytes!("../../assets/img/dinero.bmp");
const ICON_MOTO: &[u8] = include_bytes!("../../assets/img/moto.bmp");
const ICON_PESO: &[u8] = include_bytes!("../../assets/img/peso.bmp");
const ICON_UBICACION: &[u8] = include_bytes!("../../assets/img/ubicacion.bmp");

lazy_static! {
    static ref IMAGES: HashMap<&'static str, &'static [u8]> = {
        let mut m = HashMap::new();
        m.insert("camino", ICON_CAMINO);
        m.insert("cubiertos", ICON_CUBIERTOS);
        m.insert("dinero", ICON_DINERO);
        m.insert("moto", ICON_MOTO);
        m.insert("peso", ICON_PESO);
        m.insert("ubicacion", ICON_UBICACION);
        m
    };
    static ref FONTS: HashMap<&'static str, &'static [u8]> = {
        let mut m = HashMap::new();
        m.insert("bold", FONT_BOLD);
        m.insert("light", FONT_LIGHT);
        m
    };
}

pub struct ParagraphData {
    pub lines: Vec<ParrafoLine>,
    pub font_size: f32,
    pub light: bool,
    /// Si quieres, también puedes guardar el `final_y` por si te sirve para
    /// cálculos posteriores (p.ej. para saber dónde empieza el siguiente párrafo).
    ///
    pub y_inicial: f32,
    // pub final_y: f32,
}

pub struct ParrafoLine {
    pub text: String,
    // pub width_mm: f32,
    pub x_position: f32,
}

pub struct FontData<'a> {
    pub font_vec: &'a [u8],
    pub upem: f32,
    pub face: Face<'a>,
}
pub struct ImagePreMake {
    pub img: Image,
    pub trans: Box<dyn Fn(f32) -> ImageTransform>,
}

pub struct CurrentPdf {
    pub current_layer: PdfLayerReference,
    pub doc: PdfDocumentReference,
    pub font_bold: IndirectFontRef,
    pub font_light: IndirectFontRef,
}
pub struct PdfResources<'a> {
    pub bold: FontData<'a>,
    pub light: FontData<'a>,
    pub paragraphs: Vec<ParagraphData>,
    pub polygons: Vec<Box<dyn Fn(f32) -> Polygon>>,
    pub imgs: Vec<ImagePreMake>,
    pub current_pdf: Option<CurrentPdf>,
    pub page_height: f32,
}

impl<'a> PdfResources<'a> {
    pub fn new() -> Self {
        let bold_data = FONTS
            .get("bold")
            .expect("No se pudo encontrar la fuente bold");
        let face_bold: Face<'_> =
            Face::parse(bold_data, 0).expect("No se pudo cargar la fuente bold");
        let upem_bold: f32 = face_bold.units_per_em() as f32;

        let bold: FontData = FontData {
            font_vec: bold_data,
            upem: upem_bold,
            face: face_bold,
        };

        let light_data = FONTS
            .get("light")
            .expect("No se pudo encontrar la fuente light");
        let face_light: Face<'_> =
            Face::parse(light_data, 0).expect("No se pudo cargar la fuente light");
        let upem_light: f32 = face_light.units_per_em() as f32;

        let light: FontData = FontData {
            font_vec: light_data,
            upem: upem_light,
            face: face_light,
        };

        PdfResources {
            bold,
            light,
            paragraphs: Vec::new(),
            polygons: Vec::new(),
            imgs: Vec::new(),
            current_pdf: None,
            page_height: 0.0,
        }
    }
    pub fn set_img(&mut self, x: f32, y: f32, mm_x: f32, mm_y: f32, icono: &str) {
        fn px_to_mm(px: f32) -> f32 {
            (px / DPI) * 25.4
        }

        // En lugar de abrir el archivo, usamos los bytes incluidos
        let image_bytes = IMAGES.get(icono).expect("Imagen no encontrada");
        let img = Image::try_from(
            image_crate::codecs::bmp::BmpDecoder::new(Cursor::new(image_bytes))
                .expect("Error al decodificar imagen"),
        )
        .expect("Error al crear imagen");

        let base_scale_x = px_to_mm(img.image.width.0 as f32);
        let base_scale_y = px_to_mm(img.image.height.0 as f32);

        let scale_x = mm_x / base_scale_x;
        let scale_y = mm_y / base_scale_y;

        let image_transform: Box<dyn Fn(f32) -> ImageTransform> =
            Box::new(move |height: f32| ImageTransform {
                translate_x: Some(Mm(x)),
                translate_y: Some(Mm(height - y)),
                rotate: None,
                scale_x: Some(scale_x),
                scale_y: Some(scale_y),
                dpi: Some(DPI),
            });

        let img_save = ImagePreMake {
            img,
            trans: image_transform,
        };

        self.imgs.push(img_save);
    }

    pub fn set_rect(&mut self, y_inicio: f32, y_fin: f32) {
        self.polygons.push(set_linea_horizontal(y_inicio));
        self.polygons.push(set_linea_horizontal(y_fin));

        let lines_ver = set_linea_vertical(y_inicio, y_fin);
        self.polygons.push(lines_ver.0);
        self.polygons.push(lines_ver.1);
    }

    /// Crea el layout de un párrafo y lo guarda internamente en `self.paragraphs`.
    pub fn set_paragraph(
        &mut self,
        text: &str,
        font_size: f32,
        y_inicial: f32,
        max_width_mm: f32,
        tipo: i8,
        light: bool,
    ) -> f32 {
        let font_data: &FontData<'a> = if light { &self.light } else { &self.bold };

        // Llamamos a tu función de layout
        let (lines, final_y) =
            layout_parrafo(text, font_size, y_inicial, max_width_mm, tipo, font_data);

        // Guardamos el resultado en nuestro vector
        let paragraph_data = ParagraphData {
            lines,
            y_inicial,
            font_size,
            light,
            // final_y,
        };

        self.paragraphs.push(paragraph_data);

        if final_y > self.page_height {
            self.page_height = final_y;
        }
        // Podemos devolver el final_y para que el que llame sepa
        // dónde quedó la última línea (por si quieres encadenar párrafos).
        final_y
    }
    pub fn set_linea(&mut self, y: f32) {
        self.polygons.push(set_linea_horizontal(y));
    }
    pub fn set_separacion(&mut self, y: f32, icono: &str) {
        self.polygons.push(set_linea_horizontal(y + 1.5));
        self.polygons.push(set_linea_horizontal(y + 2.75));
        self.set_img(37.0, y + 5.5, 6.0, 6.0, icono);
    }
    /// Dibuja las líneas en el PDF usando el `current_layer`.
    /// Requiere los datos de las líneas previamente calculadas por `layout_parrafo`.
    pub fn draw_parrafo(
        current_layer: &PdfLayerReference,
        lines: &[ParrafoLine],
        y_inicial: f32,
        font_size: f32,
        font: &IndirectFontRef,
    ) {
        // Devolvemos la última posición X e Y usada
        // 3. Dibujamos línea a línea
        let mut y_position = y_inicial;
        // let mut last_x_position = 10.0; // o como gustes

        // 2. Interlineado
        let line_spacing = font_size * 0.4;
        for line_info in lines {
            // Dibujar la línea
            current_layer.use_text(
                &line_info.text,
                font_size,
                Mm(line_info.x_position),
                Mm(y_position),
                font,
            );

            // last_x_position = line_info.x_position;
            // Para la siguiente línea, reducimos Y
            y_position -= line_spacing;
        }
    }

    pub fn init_draw(&mut self) {
        let (doc, page, layer) = PdfDocument::new("Ticket", Mm(80.0), Mm(self.page_height), "");

        let font_bold: IndirectFontRef = doc.add_external_font(self.bold.font_vec).expect(
            "Failed to add external font. Ensure the font data is correct and file path is valid.",
        );
        let font_light: IndirectFontRef = doc.add_external_font(self.light.font_vec).expect(
            "Failed to add external font. Ensure the font data is correct and file path is valid.",
        );

        // Obtenemos la "layer" actual
        let current_layer: PdfLayerReference = doc.get_page(page).get_layer(layer);
        let pdf = CurrentPdf {
            doc,
            // page,
            current_layer,
            font_bold,
            font_light,
        };
        self.current_pdf = Some(pdf);
    }
    pub fn save(self) -> Result<Vec<u8>, String> {
        match self.current_pdf {
            Some(use_pdf) => {
                // let mut buffer: BufWriter<File> = BufWriter::new(File::create("test.pdf").unwrap());
                // use_pdf.doc.save(&mut buffer).unwrap();
                // Ok((Vec::new(), Vec::new()))

                let pdf_base = use_pdf
                    .doc
                    .save_to_bytes()
                    .map_err(|e| format!("Error al guardar el PDF base: {:?}", e))?;
                Ok(pdf_base)

                // // Obtener el último párrafo
                // let p = self
                //     .paragraphs
                //     .last()
                //     .ok_or_else(|| "No se encontró el último párrafo".to_string())?;

                // // Determinar la fuente a usar
                // let font_use: &IndirectFontRef = if p.light {
                //     &use_pdf.font_light
                // } else {
                //     &use_pdf.font_bold
                // };

                // // Dibujar el párrafo
                // Self::draw_parrafo(
                //     &use_pdf.current_layer,
                //     &p.lines,
                //     self.page_height - p.y_inicial,
                //     p.font_size,
                //     font_use,
                // );

                // let mut pdf_reimpreso = pdf_base.clone();

                // if let Some(new_content) = p.lines.last() {
                //     let additional_bytes = new_content.text.as_bytes();
                //     pdf_reimpreso.extend_from_slice(additional_bytes);
                // } else {
                //     return Err("No se encontró contenido para agregar al PDF".to_string());
                // }
                // Ok((pdf_base, pdf_reimpreso))
            }
            None => Err("No se encontró un PDF actual".to_string()),
        }
    }

    pub fn drow_all_obj(&mut self) {
        match &self.current_pdf {
            Some(use_pdf) => {
                // textos
                match &self.current_pdf {
                    Some(use_pdf) => {
                        for p in self.paragraphs.iter() {
                            let font_use: &IndirectFontRef = if p.light {
                                &use_pdf.font_light
                            } else {
                                &use_pdf.font_bold
                            };
                            Self::draw_parrafo(
                                &use_pdf.current_layer,
                                &p.lines,
                                self.page_height - p.y_inicial,
                                p.font_size,
                                font_use,
                            );
                        }
                    }
                    None => {
                        print!("Error");
                    }
                };

                // plogonos
                for poly in &self.polygons {
                    use_pdf.current_layer.add_polygon((poly)(self.page_height));
                }
                // imagenes

                for img in self.imgs.drain(..) {
                    img.img
                        .add_to_layer(use_pdf.current_layer.clone(), (img.trans)(self.page_height));
                }
            }
            None => {
                print!("Error");
            }
        };
    }
}

/// Calcula la disposición (layout) de las líneas de un párrafo,
/// sin dibujar nada todavía.
///
/// `tipo` define la alineación:
///   - < 0 => izquierda
///   - = 0 => centrada
///   - > 0 => derecha
///
/// Devuelve:
///   - `Vec<ParrafoLine>`: cada línea con su texto, ancho y X calculado
///   - `final_y`: la posición Y que queda tras colocar todas las líneas
fn layout_parrafo(
    text: &str,
    font_size: f32,
    y_inicial: f32,
    max_width_mm: f32,
    tipo: i8,
    font_data: &FontData,
) -> (Vec<ParrafoLine>, f32) {
    // 2. Calcular factor de escala de la fuente
    let scale_factor = font_size / font_data.upem;

    // 3. Dividir el texto en palabras
    let words: Vec<&str> = text.split_whitespace().collect();

    // 4. Función para medir el ancho de una palabra en mm
    let measure_word_mm = |w: &str| -> f32 {
        let width_points: f32 = w
            .chars()
            .filter_map(|c| font_data.face.glyph_index(c))
            .map(|glyph_id| font_data.face.glyph_hor_advance(glyph_id).unwrap_or(0) as f32)
            .sum::<f32>()
            * scale_factor;

        // Convertir de puntos tipográficos a mm (aprox)
        width_points * 0.352778
    };

    // 5. Recorremos las palabras para construir líneas
    let mut lines_text: Vec<String> = Vec::new();
    let mut current_line = String::new();
    let mut current_line_width: f32 = 0.0;

    let space_width_mm = measure_word_mm(" "); // ancho de un espacio

    for (i, word) in words.iter().enumerate() {
        // Ancho de la palabra actual
        let word_width = measure_word_mm(word);

        // Si la línea actual está vacía no agregamos espacio
        let sep = if i == 0 || current_line.is_empty() {
            ""
        } else {
            " "
        };
        let extra_width = if sep.is_empty() { 0.0 } else { space_width_mm };

        // Ver si cabe en la línea actual
        if current_line_width + word_width + extra_width <= max_width_mm {
            // Cabe
            if current_line.is_empty() {
                current_line = word.to_string();
            } else {
                current_line.push_str(sep);
                current_line.push_str(word);
            }
            current_line_width += word_width + extra_width;
        } else {
            // No cabe: guardamos la línea y empezamos una nueva
            lines_text.push(current_line);
            current_line = word.to_string();
            current_line_width = word_width;
        }
    }
    // Agregamos la última si quedó algo
    if !current_line.is_empty() {
        lines_text.push(current_line);
    }

    // 6. Para cada línea calculamos su ancho y su X
    let mut lines_layout = Vec::new();
    for line_str in &lines_text {
        let line_width_mm = measure_word_mm(line_str);

        let x_position = if tipo < 0 {
            // Izquierda
            -5.0 * tipo as f32
        } else if tipo == 0 {
            // Centrado en la página
            (80.0 - line_width_mm) / 2.0
        } else {
            // Derecha (asume contenedor de `max_width_mm` empezando en x=10)
            // Podrías ajustarlo a tus márgenes reales
            5.0 * tipo as f32 + (max_width_mm - line_width_mm)
        };

        let line_info = ParrafoLine {
            text: line_str.to_string(),
            // width_mm: line_width_mm,
            x_position,
        };

        lines_layout.push(line_info);
    }

    // 7. Calculamos la posición Y “final”, asumiendo un interlineado
    //    (Aunque podrías no retornarlo y que se maneje afuera).
    let line_spacing = font_size * 0.4;
    let final_y = y_inicial + (lines_layout.len() as f32 * line_spacing);

    (lines_layout, final_y)
}

pub fn format_datetime(iso_date: &str) -> (String, String) {
    // Parseamos la fecha ISO
    let datetime = DateTime::parse_from_rfc3339(iso_date).expect("Error parsing ISO date");

    // Mapa de traducción para los meses en español
    let mut months = HashMap::new();
    months.insert("Jan", "Ene");
    months.insert("Feb", "Feb");
    months.insert("Mar", "Mar");
    months.insert("Apr", "Abr");
    months.insert("May", "May");
    months.insert("Jun", "Jun");
    months.insert("Jul", "Jul");
    months.insert("Aug", "Ago");
    months.insert("Sep", "Sep");
    months.insert("Oct", "Oct");
    months.insert("Nov", "Nov");
    months.insert("Dec", "Dic");

    // Formateamos la fecha como "DD MMM"
    let raw_date = datetime.format("%d %b").to_string();
    let date = raw_date
        .split_whitespace()
        .map(|part| months.get(part).unwrap_or(&part).to_string())
        .collect::<Vec<String>>()
        .join(" ");

    // Formateamos la hora como "HH:MM"
    let time = datetime.format("%H:%M").to_string();

    (date, time)
}

pub fn format_clp(amount: i32) -> String {
    let mut formatted = String::new();
    let amount_str = amount.to_string();
    let mut count = 0;

    for ch in amount_str.chars().rev() {
        if count > 0 && count % 3 == 0 {
            formatted.push('.');
        }
        formatted.push(ch);
        count += 1;
    }

    formatted = formatted.chars().rev().collect();
    format!("${}", formatted)
}

pub fn set_linea_horizontal(y: f32) -> Box<dyn Fn(f32) -> Polygon> {
    // let i:f32  = (resources.page_height - altura) as f32;
    let poligono: Box<dyn Fn(f32) -> Polygon> = Box::new(move |height: f32| {
        let i = height - y;
        let l = 5.0;
        let r = 75.0;
        let points = vec![
            (Point::new(Mm(l), Mm(i)), false),
            (Point::new(Mm(l), Mm(i)), false),
            (Point::new(Mm(r), Mm(i)), false),
            (Point::new(Mm(r), Mm(i)), false),
        ];

        let line = Polygon {
            rings: vec![points],
            mode: PaintMode::FillStroke,
            winding_order: WindingOrder::NonZero,
        };
        line
    });
    poligono
}

pub fn set_linea_vertical(
    y1: f32,
    y2: f32,
) -> (Box<dyn Fn(f32) -> Polygon>, Box<dyn Fn(f32) -> Polygon>) {
    let pol_l: Box<dyn Fn(f32) -> Polygon> = Box::new(move |height: f32| {
        let r = 75.0;
        let points_r = vec![
            (Point::new(Mm(r), Mm(height - y1)), false),
            (Point::new(Mm(r), Mm(height - y2)), false),
        ];
        let line_l = Polygon {
            rings: vec![points_r],
            mode: PaintMode::FillStroke,
            winding_order: WindingOrder::NonZero,
        };
        line_l
    });

    let pol_r: Box<dyn Fn(f32) -> Polygon> = Box::new(move |height: f32| {
        let l = 5.0;
        let points_l = vec![
            (Point::new(Mm(l), Mm(height - y1)), false),
            (Point::new(Mm(l), Mm(height - y2)), false),
        ];
        let line_r = Polygon {
            rings: vec![points_l],
            mode: PaintMode::FillStroke,
            winding_order: WindingOrder::NonZero,
        };
        line_r
    });
    (pol_l, pol_r)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fonts_included() {
        let bold_font = include_bytes!("../../assets/fonts/segoe-ui-bold.ttf");
        let light_font = include_bytes!("../../assets/fonts/segoe-ui.ttf");

        assert!(!bold_font.is_empty(), "Bold font data should not be empty");
        assert!(
            !light_font.is_empty(),
            "Light font data should not be empty"
        );
    }
}

/*

const getTicketKitchen = async ({ order, isCopy }) => {
    const tz = order.Fechas.Tz
    const _currencyFormatterOptions = currencyFormatterOptions['CLP']
    const address = order.PickUp.Direccion.split(',')[0]


    const encoder = new EscPosEncoder()
    const totalCharWitdth = 33
    const totalCharWitdthDoubleSize = Math.floor(totalCharWitdth / 2)
    const spaces = ' '.repeat(totalCharWitdth)
    const separator = '-'.repeat(totalCharWitdth)

    const bufferHexDoubleSize = turn => [0x1b, 0x21, turn ? 48 : 0]
    const bufferHexDoubleHeight = turn => [0x1b, 0x21, turn ? 16 : 0]

    encoder
        .initialize()
        .align('center')
        .size('small')
        .line(order.Comercio.Nombre)
        .size('normal')
        .size('small')
        .line(order.Plataforma.Nombre)
        .size('normal')
        .line(separator)
        .raw(bufferHexDoubleSize(true))
        .bold(true)
        .line(order.TipoEntrega.Id === 1 ? 'DELIVERY' : 'RETIRO')
        .bold(false)
        .raw(bufferHexDoubleSize(false))
        // hora salida cocina
        .line(separator)
        .align('left')
        .line('Salida Cocina')
        .align('right')
        .raw(bufferHexDoubleSize(true))
        .line(moment.tz(order.Fechas.FechaSalidaCocinaEstimada, tz).format('HH:mm'))
        .raw(bufferHexDoubleSize(false))
        // correlativo
        .align('center')
        .line(separator)
        .raw(bufferHexDoubleSize(true))
        .bold(true)
        .line(order.Correlativo > 0 && order.Plataforma.Codigo === 'AGIL' ? order.Correlativo : '')
        .bold(false)
        .raw(bufferHexDoubleSize(false))
        //codigo + corelativo
        .line(separator)
        .raw(bufferHexDoubleSize(true))
        .bold(true)
        .line(`#${order.Codigo}${order.Correlativo > 0 ? `-${order.Correlativo}` : ''}`)
        .bold(false)
        .raw(bufferHexDoubleSize(false))
        // cliente
        .align('center')
        .line(separator)
        .bold(true)
        .raw(bufferHexDoubleSize(true))
        .line(`${order.Cliente.Nombre} ${order.Cliente.Apellido || ''}`)
        .raw(bufferHexDoubleSize(false))
        .bold(false)
        .line(separator)
    if (order.Comentario) {
        encoder.bold(true).line('Comentario del Cliente:')
            .bold(false).line(`"${eliminarDiacriticos(order.Comentario)}"`)
            .raw(bufferHexDoubleSize(false))
            .bold(false)
            .line(separator)
    }

    order.Items.forEach(item => {
        encoder.align('left')
            .raw(bufferHexDoubleHeight(true))
            .bold(true)
            .line(`${item.Cantidad} X ${eliminarDiacriticos(item.Nombre)}`) // .toUpperCase()
            .bold(false)
            .raw(bufferHexDoubleHeight(false));

        item.opciones.map(opt => {
            encoder
                .line(`-${eliminarDiacriticos(opt.Modificador)}`)
                .line(`${opt.Cantidad} X ${eliminarDiacriticos(opt.Opcion)}`)
        })

        encoder.line(spaces)
        if (item.Comentario) {
            encoder.bold(true).line('Comentario del Producto:')
                .bold(false).line(`"${eliminarDiacriticos(item.Comentario)}"`).line(spaces)
        }

    })

    encoder
        .align('center')
        .line(separator)
        .line('     ')
        .line('     ')
        .line('     ')
        .line('     ')
        .line('     ')
        .line('     ')
        .cut('partial')
        .align('left')
    // console.log(encoder.encode())
    const fileBuffer = Buffer.from(encoder.encode().buffer)

    const { Location } = await S3.upload({
        Bucket: process.env.S3_BUCKET,
        Key: getS3KeyTicketOrder({ order, format: 'KITCHEN', extension: 'txt', isCopy }),
        Body: fileBuffer,
        ACL: 'public-read',
    })
        .promise()
        .catch(err => {
            // console.error('UPLOAD FILE S3 ERR', err)
            return { Location: null }
        })

    return Location
}


*/
