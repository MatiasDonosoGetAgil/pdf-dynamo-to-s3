extern crate printpdf;
use std::convert::From;

use chrono::{DateTime, Local};
use unicode_normalization::char::is_combining_mark;
use unicode_normalization::UnicodeNormalization;

use serde::{Deserialize, Serialize};
mod pdf;
use pdf::{format_clp, format_datetime, PdfResources};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Payload {
    pub order: IOrder,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IOrder {
    #[serde(rename = "IdPedido")]
    pub id_pedido: i32,
    #[serde(rename = "IdSucursal")]
    pub id_sucursal: i32,
    #[serde(rename = "IdPublico")]
    pub id_publico: String,
    #[serde(rename = "VersionPOS")]
    pub version_pos: String,
    #[serde(rename = "Codigo")]
    pub codigo: String,
    #[serde(rename = "Correlativo")]
    pub correlativo: i32,
    #[serde(rename = "Plataforma")]
    pub plataforma: Plataforma,
    #[serde(rename = "TipoPedido")]
    pub tipo_pedido: TipoPedido,
    #[serde(rename = "SubTotal")]
    pub sub_total: i32,
    #[serde(rename = "GastosEnvio")]
    pub gastos_envio: i32,
    #[serde(rename = "DsctoCuponSubtotal")]
    pub dscto_cupon_subtotal: f32,
    #[serde(rename = "DsctoCuponGastoEnvio")]
    pub dscto_cupon_gasto_envio: f32,
    #[serde(rename = "DsctoPuntos")]
    pub dscto_puntos: f32,
    #[serde(rename = "CuotaServicioAgil")]
    pub cuota_servicio_agil: f32,
    #[serde(rename = "Propina")]
    pub propina: f32,
    #[serde(rename = "PropinaAgil")]
    pub propina_agil: f32,
    #[serde(rename = "TotalOC")]
    pub total_oc: i32,
    #[serde(rename = "Comentario")]
    pub comentario: Option<String>,
    #[serde(rename = "TieneHijos")]
    pub tiene_hijos: bool,
    #[serde(rename = "IdPedidoBase")]
    pub id_pedido_base: Option<String>,
    #[serde(rename = "IdCarro")]
    pub id_carro: i32,
    #[serde(rename = "Estado")]
    pub estado: Estado,
    #[serde(rename = "ComentarioFlash")]
    pub comentario_flash: Option<String>,
    #[serde(rename = "EstadoDelivery")]
    pub estado_delivery: EstadoDelivery,
    #[serde(rename = "isDelivery")]
    pub is_delivery: bool,
    #[serde(rename = "isScheduled")]
    pub is_scheduled: bool,
    #[serde(rename = "isOwnDelivery")]
    pub is_own_delivery: bool,
    #[serde(rename = "TipoFechaEntrega")]
    pub tipo_fecha_entrega: TipoFechaEntrega,
    #[serde(rename = "TipoEntrega")]
    pub tipo_entrega: TipoEntrega,
    #[serde(rename = "Cupones")]
    pub cupones: Vec<Cupon>,
    #[serde(rename = "Pago")]
    pub pago: Pago,
    #[serde(rename = "Sucursal")]
    pub sucursal: Sucursal,
    #[serde(rename = "Comercio")]
    pub comercio: Comercio,
    #[serde(rename = "PickUp")]
    pub pick_up: PickUp,
    #[serde(rename = "DropOff")]
    pub drop_off: DropOff,
    #[serde(rename = "Courier")]
    pub courier: Courier,
    #[serde(rename = "Cliente")]
    pub cliente: Cliente,
    #[serde(rename = "Fechas")]
    pub fechas: Fechas,
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
    #[serde(rename = "Urls")]
    pub urls: Vec<Url>,
    #[serde(rename = "Proveedores")]
    pub proveedores: Vec<String>,
    #[serde(rename = "AlternativaDelivery")]
    pub alternativa_delivery: Vec<AlternativaDelivery>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Plataforma {
    #[serde(rename = "Codigo")]
    pub codigo: String,
    #[serde(rename = "Logo")]
    pub logo: String,
    #[serde(rename = "Nombre")]
    pub nombre: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TipoPedido {
    #[serde(rename = "Codigo")]
    pub codigo: String,
    #[serde(rename = "Nombre")]
    pub nombre: String,
    #[serde(rename = "Prefijo")]
    pub prefijo: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Estado {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Nombre")]
    pub nombre: String,
    #[serde(rename = "Codigo")]
    pub codigo: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EstadoDelivery {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Nombre")]
    pub nombre: String,
    #[serde(rename = "Codigo")]
    pub codigo: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TipoFechaEntrega {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Tipo")]
    pub tipo: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TipoEntrega {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Tipo")]
    pub tipo: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Cupon {
    #[serde(rename = "Codigo")]
    pub codigo: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Pago {
    #[serde(rename = "IdPublico")]
    pub id_publico: String,
    #[serde(rename = "Tarjeta")]
    pub tarjeta: Option<String>,
    #[serde(rename = "IdPago")]
    pub id_pago: i64,
    #[serde(rename = "MedioPago")]
    pub medio_pago: MedioPago,
    #[serde(rename = "TipoPago")]
    pub tipo_pago: TipoPago,
    #[serde(rename = "EstadoPago")]
    pub estado_pago: EstadoPago,
    #[serde(rename = "EstadoReembolso")]
    pub estado_reembolso: String,
    #[serde(rename = "FechaReembolso")]
    pub fecha_reembolso: Option<String>,
    #[serde(rename = "MontoReembolso")]
    pub monto_reembolso: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MedioPago {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Nombre")]
    pub nombre: String,
    #[serde(rename = "Logo")]
    pub logo: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TipoPago {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Tipo")]
    pub tipo: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EstadoPago {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Estado")]
    pub estado: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Sucursal {
    #[serde(rename = "IdSucursal")]
    pub id_sucursal: i64,
    #[serde(rename = "Nombre")]
    pub nombre: String,
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "Telefono")]
    pub telefono: String,
    #[serde(rename = "Direccion")]
    pub direccion: String,
    #[serde(rename = "Lat")]
    pub lat: String,
    #[serde(rename = "Lng")]
    pub lng: String,
    #[serde(rename = "Comuna")]
    pub comuna: String,
    #[serde(rename = "Ciudad")]
    pub ciudad: String,
    #[serde(rename = "Tz")]
    pub tz: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Comercio {
    #[serde(rename = "IdComercio")]
    pub id_comercio: i64,
    #[serde(rename = "Nombre")]
    pub nombre: String,
    #[serde(rename = "CDN")]
    pub cdn: String,
    #[serde(rename = "Moneda")]
    pub moneda: String,
    #[serde(rename = "Iva")]
    pub iva: String,
    #[serde(rename = "Logo")]
    pub logo: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PickUp {
    #[serde(rename = "Lat")]
    pub lat: String,
    #[serde(rename = "Lng")]
    pub lng: String,
    #[serde(rename = "Direccion")]
    pub direccion: String,
    #[serde(rename = "Ciudad")]
    pub ciudad: String,
    #[serde(rename = "Nombre")]
    pub nombre: String,
    #[serde(rename = "Evidencia")]
    pub evidencia: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DropOff {
    #[serde(rename = "Lat")]
    pub lat: String,
    #[serde(rename = "Lng")]
    pub lng: String,
    #[serde(rename = "Direccion")]
    pub direccion: String,
    #[serde(rename = "Ciudad")]
    pub ciudad: String,
    #[serde(rename = "Distancia")]
    pub distancia: i32,
    #[serde(rename = "Nombre")]
    pub nombre: String,
    #[serde(rename = "TipoEntrega")]
    pub tipo_entrega: String,
    #[serde(rename = "Evidencia")]
    pub evidencia: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Courier {
    #[serde(rename = "IdCourier")]
    pub id_courier: Option<i32>,
    #[serde(rename = "Plataforma")]
    pub plataforma: Option<String>,
    #[serde(rename = "Logo")]
    pub logo: Option<String>,
    #[serde(rename = "IdExterno")]
    pub id_externo: Option<String>,
    #[serde(rename = "Llave")]
    pub llave: i64,
    #[serde(rename = "IdRider")]
    pub id_rider: Option<String>,
    #[serde(rename = "Nombre")]
    pub nombre: Option<String>,
    #[serde(rename = "Telefono")]
    pub telefono: Option<String>,
    #[serde(rename = "Imagen")]
    pub imagen: String,
    #[serde(rename = "Lat")]
    pub lat: String,
    #[serde(rename = "Lng")]
    pub lng: String,
    #[serde(rename = "Distancia")]
    pub distancia: Option<i32>,
    #[serde(rename = "Bitacora")]
    pub bitacora: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Cliente {
    #[serde(rename = "IdUsuario")]
    pub id_usuario: i64,
    #[serde(rename = "Nombre")]
    pub nombre: String,
    #[serde(rename = "Apellido")]
    pub apellido: String,
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "Telefono")]
    pub telefono: String,
    #[serde(rename = "PrefijoTelefonico")]
    pub prefijo_telefonico: i32,
    #[serde(rename = "Direccion")]
    pub direccion: String,
    #[serde(rename = "Lat")]
    pub lat: String,
    #[serde(rename = "Lng")]
    pub lng: String,
    #[serde(rename = "Nro")]
    pub nro: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Fechas {
    #[serde(rename = "Tz")]
    pub tz: String,
    #[serde(rename = "FechaCreacion")]
    pub fecha_creacion: String,
    #[serde(rename = "FechaPago")]
    pub fecha_pago: String,
    #[serde(rename = "FechaAceptacionEstimada")]
    pub fecha_aceptacion_estimada: String,
    #[serde(rename = "FechaAceptacionReal")]
    pub fecha_aceptacion_real: Option<String>,
    #[serde(rename = "FechaSalidaCocinaEstimada")]
    pub fecha_salida_cocina_estimada: String,
    #[serde(rename = "FechaSalidaCocinaReal")]
    pub fecha_salida_cocina_real: Option<String>,
    #[serde(rename = "FechaEntregaMin")]
    pub fecha_entrega_min: String,
    #[serde(rename = "FechaEntregaMax")]
    pub fecha_entrega_max: String,
    #[serde(rename = "FechaEntregaReal")]
    pub fecha_entrega_real: Option<String>,
    #[serde(rename = "FechaSolicitudCourierEstimada")]
    pub fecha_solicitud_courier_estimada: String,
    #[serde(rename = "FechaSolicitudCourierReal")]
    pub fecha_solicitud_courier_real: Option<String>,
    #[serde(rename = "FechaLlegadaCourierEstimada")]
    pub fecha_llegada_courier_estimada: Option<String>,
    #[serde(rename = "FechaLlegadaCourierReal")]
    pub fecha_llegada_courier_real: Option<String>,
    #[serde(rename = "FechaRechazoMaximoSinCajero")]
    pub fecha_rechazo_maximo_sin_cajero: Option<String>,
    #[serde(rename = "FechaRechazoReal")]
    pub fecha_rechazo_real: Option<String>,
    #[serde(rename = "FechaModificacion")]
    pub fecha_modificacion: Option<String>,
    #[serde(rename = "FechaPrimeraEvidencia")]
    pub fecha_primera_evidencia: Option<String>,
    #[serde(rename = "FechaSegundaEvidencia")]
    pub fecha_segunda_evidencia: Option<String>,
    #[serde(rename = "TiempoPreparacion")]
    pub tiempo_preparacion: String,
    #[serde(rename = "TiempoCocina")]
    pub tiempo_cocina: String,
    #[serde(rename = "TiempoDelivery")]
    pub tiempo_delivery: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Item {
    #[serde(rename = "IdProducto")]
    pub id_producto: i64,
    #[serde(rename = "IdProductoExterno")]
    pub id_producto_externo: Option<String>,
    #[serde(rename = "SKU")]
    pub sku: String,
    #[serde(rename = "Nombre")]
    pub nombre: String,
    #[serde(rename = "Descripcion")]
    pub descripcion: String,
    #[serde(rename = "Imagen")]
    pub imagen: String,
    #[serde(rename = "Categoria")]
    pub categoria: Vec<Categoria>,
    #[serde(rename = "Cantidad")]
    pub cantidad: f32,
    #[serde(rename = "Precio")]
    pub precio: String,
    #[serde(rename = "PrecioFinal")]
    pub precio_final: String,
    #[serde(rename = "Total")]
    pub total: String,
    #[serde(rename = "TotalFinal")]
    pub total_final: String,
    #[serde(rename = "Comentario")]
    pub comentario: Option<String>,
    #[serde(rename = "EsAgrupacion")]
    pub es_agrupacion: Option<bool>,
    #[serde(rename = "NombreAgrupacion")]
    pub nombre_agrupacion: Option<String>,
    #[serde(rename = "CorrelativoAgrupacion")]
    pub correlativo_agrupacion: i32,
    #[serde(rename = "opciones")]
    pub opciones: Vec<IOpciones>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Categoria {
    #[serde(rename = "IdCategoria")]
    pub id_categoria: i64,
    #[serde(rename = "Nombre")]
    pub nombre: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IOpciones {
    #[serde(rename = "Modificador")]
    pub modificador: String,
    #[serde(rename = "Cantidad")]
    pub cantidad: i32,
    #[serde(rename = "Opcion")]
    pub opcion: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Url {
    #[serde(rename = "Tipo")]
    pub tipo: String,
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AlternativaDelivery {
    #[serde(rename = "IdCourier")]
    pub id_courier: i32,
    #[serde(rename = "Nombre")]
    pub nombre: String,
    #[serde(rename = "Total")]
    pub total: f32,
    #[serde(rename = "CuotaServicioCourier")]
    pub cuota_servicio_courier: CuotaServicioCourier,
    #[serde(rename = "TipoTarifa")]
    pub tipo_tarifa: Option<String>,
    #[serde(rename = "Distancia")]
    pub distancia: Option<i32>,
    #[serde(rename = "DistanciaTarifaDesde")]
    pub distancia_tarifa_desde: Option<i32>,
    #[serde(rename = "DistanciaTarifaHasta")]
    pub distancia_tarifa_hasta: Option<i32>,
    #[serde(rename = "PrecioBaseClienteNeto")]
    pub precio_base_cliente_neto: Option<i32>,
    #[serde(rename = "PrecioBaseRiderNeto")]
    pub precio_base_rider_neto: Option<i32>,
    #[serde(rename = "PrecioExtraCliente")]
    pub precio_extra_cliente: Option<i32>,
    #[serde(rename = "PrecioExtraRider")]
    pub precio_extra_rider: Option<i32>,
    #[serde(rename = "CuotaServicioAgilNeto")]
    pub cuota_servicio_agil_neto: Option<i32>,
    #[serde(rename = "CuotaServicioAgilIva")]
    pub cuota_servicio_agil_iva: Option<i32>,
    #[serde(rename = "CuotaServicioAgilBruto")]
    pub cuota_servicio_agil_bruto: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CuotaServicioCourier {
    #[serde(rename = "CuotaServicioNeto")]
    pub cuota_servicio_neto: Option<i32>,
    #[serde(rename = "CuotaServicioIva")]
    pub cuota_servicio_iva: Option<f32>,
    #[serde(rename = "CuotaServicioBruto")]
    pub cuota_servicio_bruto: Option<f32>,
    #[serde(rename = "GastoEnvioNeto")]
    pub gasto_envio_neto: Option<i32>,
    #[serde(rename = "GastoEnvioIva")]
    pub gasto_envio_iva: Option<i32>,
    #[serde(rename = "GastoEnvioBruto")]
    pub gasto_envio_bruto: Option<i32>,
    #[serde(rename = "Desde")]
    pub desde: Option<i32>,
    #[serde(rename = "Hasta")]
    pub hasta: Option<i32>,
}

struct EscPos {
    buffer: Vec<u8>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum EscPosError {
    IoError(std::io::Error),
    EncodingError(String),
}

impl From<std::io::Error> for EscPosError {
    fn from(error: std::io::Error) -> Self {
        EscPosError::IoError(error)
    }
}

impl EscPos {
    pub fn new() -> Self {
        let mut printer = Self { buffer: Vec::new() };
        printer.buffer.extend_from_slice(&[0x1B, 0x40]); // ESC @
        printer
    }

    pub fn align_left(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1B, 0x61, 0x00]); // ESC a 0
        self
    }

    pub fn align_center(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1B, 0x61, 0x01]); // ESC a 1
        self
    }

    pub fn align_right(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1B, 0x61, 0x02]); // ESC a 2
        self
    }

    pub fn emphasis(&mut self, on: bool) -> &mut Self {
        self.buffer
            .extend_from_slice(&[0x1B, 0x45, if on { 1 } else { 0 }]); // ESC E n
        self
    }

    pub fn double_height(&mut self, on: bool) -> &mut Self {
        self.buffer
            .extend_from_slice(&[0x1B, 0x21, if on { 16 } else { 0 }]); // ESC ! n
        self
    }

    pub fn double_size(&mut self, on: bool) -> &mut Self {
        self.buffer
            .extend_from_slice(&[0x1B, 0x21, if on { 48 } else { 0 }]); // ESC ! n
        self
    }

    pub fn small_text(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1B, 0x4D, 0x01]); // ESC M n
        self
    }

    pub fn normal_text(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1B, 0x4D, 0x00]); // ESC M n
        self
    }
    #[allow(dead_code)]
    pub fn feed(&mut self, lines: u8) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1B, 0x64, lines]); // ESC d n
        self
    }

    pub fn partial_cut(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1D, 0x56, 0x01]); // GS V n (partial cut)
        self
    }

    pub fn text(&mut self, text: &str) -> &mut Self {
        self.buffer.extend_from_slice(text.as_bytes());
        self.buffer.extend_from_slice(b"\n"); // Agregar salto de línea automático
        self
    }

    pub fn spaces(&mut self, count: usize) -> &mut Self {
        self.buffer.extend_from_slice(" ".repeat(count).as_bytes());
        self.buffer.extend_from_slice(b"\n");
        self
    }
    pub fn separator(&mut self, count: usize) -> &mut Self {
        self.buffer.extend_from_slice("-".repeat(count).as_bytes());
        self.buffer.extend_from_slice(b"\n");
        self.spaces(count);
        self
    }
    pub fn into_vec(self) -> Vec<u8> {
        self.buffer
    }
}

fn eliminar_diacriticos(texto: &str) -> String {
    texto
        .nfd()
        .filter(|c| !is_combining_mark(*c))
        .collect::<String>()
}

pub fn get_ticket_kitchen(order: &IOrder, is_copy: bool) -> Result<Vec<u8>, String> {
    let total_char_width = 40;
    // let total_char_width_double_size = total_char_width / 2;
    // let spaces = " ".repeat(total_char_width);
    // let separator = "-".repeat(total_char_width);
    let mut printer = EscPos::new();
    // Reimpreso
    if is_copy {
        printer
            .align_center()
            .small_text()
            .text("REIMPRESO")
            .separator(total_char_width);
    }
    // Header
    printer
        .partial_cut()
        .align_center()
        .small_text()
        .text(&order.comercio.nombre)
        .normal_text()
        .small_text()
        .text(&order.plataforma.nombre)
        .normal_text()
        // .text(&separator)
        .separator(total_char_width)
        .spaces(total_char_width);

    // Entrega
    printer
        .double_size(true)
        .emphasis(true)
        .text(if order.tipo_entrega.id == 1 {
            "DELIVERY"
        } else {
            "RETIRO"
        })
        .emphasis(false)
        .double_size(false)
        // .text(&separator)
        .separator(total_char_width);

    // Salida cocina
    printer
        .align_left()
        .text(" Salida Cocina")
        .align_right()
        .double_size(true)
        .text(&format!(
            "{}",
            DateTime::parse_from_rfc3339(&order.fechas.fecha_salida_cocina_estimada) // ojo
                .unwrap()
                .with_timezone(&Local)
                .format("%H:%M")
        ))
        .double_size(false);

    // Correlativo
    printer
        .align_center()
        // .text(&separator)
        .separator(total_char_width)
        .double_size(true)
        .emphasis(true)
        .text(&order.correlativo.to_string())
        .emphasis(false)
        .double_size(false)
        // .text(&separator)
        .separator(total_char_width);

    // Código
    printer
        .double_size(true)
        .emphasis(true)
        .text(&format!(
            "#{}{}",
            order.codigo,
            if order.correlativo > 0 {
                format!("-{}", order.correlativo)
            } else {
                "".to_string()
            }
        ))
        .emphasis(false)
        .double_size(false);

    // Cliente
    printer
        .align_center()
        // .text(&separator)
        .separator(total_char_width)
        .emphasis(true)
        .double_size(true)
        .text(&format!(
            "{} {}",
            order.cliente.nombre, order.cliente.apellido
        ))
        .double_size(false)
        .emphasis(false)
        // .text(&separator)
        .separator(total_char_width);

    // Comentario del cliente
    if let Some(comentario) = &order.comentario {
        printer
            .emphasis(true)
            .text("Comentario del Cliente:")
            .emphasis(false)
            .text(&format!("\"{}\"", eliminar_diacriticos(comentario)))
            .double_size(false)
            .emphasis(false)
            // .text(&separator)
            .separator(total_char_width);
    }

    // Items
    for item in &order.items {
        printer
            .align_left()
            .double_height(true)
            .double_size(true)
            .emphasis(true)
            .text(&format!(
                " {} X {}",
                item.cantidad,
                eliminar_diacriticos(&item.nombre).to_uppercase()
            ))
            .emphasis(false);

        for opt in &item.opciones {
            printer
                .text(&format!(" - {}", eliminar_diacriticos(&opt.modificador)))
                .text(&format!(
                    " {} X {}",
                    opt.cantidad,
                    eliminar_diacriticos(&opt.opcion)
                ))
                .spaces(total_char_width);
        }

        // printer.text(&spaces);
        if let Some(comentario) = &item.comentario {
            printer
                .spaces(total_char_width / 2) //
                .emphasis(true)
                .text("Comentario del Producto:")
                .emphasis(false)
                .text(&format!("\"{}\"", eliminar_diacriticos(comentario)))
                // .text(&spaces);
                .spaces(total_char_width / 2);
        }
    }

    // Final del ticket
    printer
        .align_center()
        .double_height(false)
        .double_size(false)
        // .text(&separator)
        .separator(total_char_width)
        .spaces(5)
        .spaces(5)
        .spaces(5)
        .spaces(5)
        .spaces(5)
        .spaces(5)
        .partial_cut()
        .align_left();

    Ok(printer.into_vec())
}

pub fn get_ticket_pdf(orden: &IOrder, is_copy: bool) -> Result<Vec<u8>, String> {
    let mut pdf = PdfResources::new();
    let mut y_actual = 0.;
    // CUERPO 0: header

    // comercio nombre
    let comercio_nombre = &orden.comercio.nombre;
    y_actual = pdf.set_paragraph(comercio_nombre, 20.0, y_actual + 12.0, 70.0, 0, false);

    // plataforma nombre
    let plataforma_nombre = &orden.plataforma.nombre;
    y_actual = pdf.set_paragraph(plataforma_nombre, 16.0, y_actual + 0.0, 70.0, 0, false);
    // moto
    if orden.tipo_entrega.id == 1 {
        pdf.set_img(5.0, y_actual + 4.0, 10.0, 10.0, "moto");
    } else {
        pdf.set_img(5.0, y_actual + 4.0, 10.0, 10.0, "camino");
    }

    // nuestro (si es reparto propio)
    if matches!(orden.courier.id_courier, Some(-2)) {
        let nuestro_string = String::from("*NUESTRO*");
        let nuestro: &String = &nuestro_string;
        pdf.set_paragraph(&nuestro, 16.0, y_actual + 10.0, 70.0, -1, false);
    }
    let entrega_programada = if orden.tipo_fecha_entrega.id == 1 {
        false
    } else {
        true
    };
    // correlativo
    // si es programado entonces se antepone el "P" al codigo
    let mut correlativo_string = orden.correlativo.to_string();
    if entrega_programada {
        correlativo_string.insert(0, 'P');
        // prgramado
        let programado_string = String::from("PROG");
        pdf.set_paragraph(&programado_string, 16.0, y_actual + 18.0, 70.0, -1, false);
    }

    // codigo pedido
    let codigo_pedido = String::from("#") + &orden.codigo;
    pdf.set_paragraph(&codigo_pedido, 16.0, y_actual + 18.0, 70.0, 1, true);
    y_actual = pdf.set_paragraph(&correlativo_string, 50.0, y_actual + 10.0, 70.0, 1, false);

    // hora salida cociona
    let salida_cocina = format_datetime(&orden.fechas.fecha_salida_cocina_estimada.as_ref());
    pdf.set_paragraph(&salida_cocina.1, 32.0, y_actual, 70.0, 1, false);

    // salida cocina
    let salida_cocina_string = String::from("Salida Cocina");
    y_actual = pdf.set_paragraph(&salida_cocina_string, 12.0, y_actual, 70.0, -1, true);
    y_actual += 5.0;
    // Cliente nombre
    pdf.set_linea(y_actual - 4.0);
    y_actual = pdf.set_paragraph(&orden.cliente.nombre, 24.0, y_actual + 4.0, 70.0, 0, false);

    if matches!(orden.courier.id_courier, Some(-2)) {
        let cliente_telefono = &orden.cliente.telefono;
        y_actual = pdf.set_paragraph(cliente_telefono, 24.0, y_actual + 4.0, 70.0, 0, false);
    }

    // ubicacion
    pdf.set_separacion(y_actual - 4.0, "ubicacion");
    // si es delivery
    y_actual += 4.0;
    let direccion = if orden.tipo_entrega.id == 1 {
        &orden.drop_off.direccion.split(',').next().unwrap_or("")
    } else {
        orden.sucursal.nombre.as_str()
    };
    y_actual = pdf.set_paragraph(direccion, 14.0, y_actual + 5.0, 50.0, 0, false);
    let tipo_entrega = &orden.drop_off.tipo_entrega;
    y_actual = pdf.set_paragraph(tipo_entrega, 14.0, y_actual + 3.0, 80.0, 0, true);

    // hora pago
    let static_hora_pago = String::from("Hora de Pago");
    pdf.set_paragraph(&static_hora_pago, 14.0, y_actual + 2.0, 70.0, -1, true);
    let fecha_pago = format_datetime(&orden.fechas.fecha_pago.as_ref());
    let str_fecha_entrega = fecha_pago.0 + ". - " + &fecha_pago.1;
    y_actual = pdf.set_paragraph(&str_fecha_entrega, 14.0, y_actual + 2.0, 70.0, 1, true);
    // hora entrega
    let static_hora_entrega = String::from("Hora de Entrega");
    pdf.set_paragraph(&static_hora_entrega, 14.0, y_actual + 1.0, 70.0, -1, true);
    let fecha_entrega = format_datetime(&orden.fechas.fecha_entrega_min.as_ref());
    let str_fecha_entrega = fecha_entrega.0 + ". - " + &fecha_entrega.1;
    y_actual = pdf.set_paragraph(&str_fecha_entrega, 14.0, y_actual + 1.0, 70.0, 1, true);
    let inicio_rect = y_actual - 3.0;
    // comentario cliente
    if let Some(comentario_cliente) = orden.comentario.as_ref() {
        let static_comentario_cliente = String::from(" Comentario del Cliente: ");
        y_actual = pdf.set_paragraph(
            &static_comentario_cliente,
            14.0,
            y_actual + 2.0,
            60.0,
            -2,
            false,
        );
        // agregar comillas dobles al final e inicio del texto
        let comentario: String = " \"".to_string() + comentario_cliente + "\"";

        y_actual = pdf.set_paragraph(&comentario, 16.0, y_actual + 1.0, 68.0, 0, true);
    }
    pdf.set_rect(inicio_rect, y_actual - 2.0);
    y_actual += 2.0;
    pdf.set_separacion(y_actual, "cubiertos");
    let mut precio_total = 0;
    y_actual += 5.0;
    // CUERPO 2: pedidos
    for item in &orden.items {
        let num_precio = (item.precio.parse::<f32>().unwrap() * item.cantidad) as i32;
        precio_total += num_precio;
        let precio = format_clp(num_precio);
        pdf.set_paragraph(&precio, 13.0, y_actual + 5.0, 70.0, 1, false);
        y_actual = pdf.set_paragraph(
            &(item.cantidad.to_string() + " X " + &item.nombre),
            13.0,
            y_actual + 5.0,
            60.0,
            -1,
            false,
        );
        for modi in &item.opciones {
            y_actual = pdf.set_paragraph(
                &("- ".to_string() + &modi.modificador),
                13.0,
                y_actual + 2.0,
                70.0,
                -2,
                true,
            );
            y_actual = pdf.set_paragraph(
                &("".to_string() + &modi.cantidad.to_string() + " X   " + &modi.opcion),
                13.0,
                y_actual + 0.0,
                70.0,
                -2,
                true,
            );
        }
        if let Some(comentario) = &item.comentario {
            if comentario != "" {
                let ped_inicio_rect = y_actual - 3.0;
                // comentario cliente
                let static_comentario_cliente = String::from(" Comentario del Cliente: ");
                y_actual = pdf.set_paragraph(
                    &static_comentario_cliente,
                    14.0,
                    y_actual + 2.0,
                    60.0,
                    -2,
                    false,
                );
                // agregar comillas dobles al final e inicio del texto
                let ped_comentario: String = " \"".to_string() + comentario + "\"";
                y_actual = pdf.set_paragraph(&ped_comentario, 16.0, y_actual + 1.0, 68.0, 0, true);
                pdf.set_rect(ped_inicio_rect, y_actual - 2.0);
            }
        }
    }

    // FOOTER: pagos
    pdf.set_separacion(y_actual, "dinero");

    let descuento_monto: (f32, bool, String) = // bool es si es cupon de gasto envio o no
        if orden.dscto_cupon_gasto_envio > 0.0 {
            (
                orden.dscto_cupon_gasto_envio,
                true,
                "Descuento (".to_string()
                    + &orden.cupones[0].codigo
                    + ")",
            )
        } else if orden.dscto_cupon_subtotal > 0.0 {
            (
                orden.dscto_cupon_subtotal,
                false,
                "Descuento".to_string(),
            )
        } else {
            (
                orden.dscto_puntos,
                false,
                "Puntos".to_string(),
            )
        };

    let descuento_oferta = orden.sub_total - precio_total;
    let gastos_envio = orden.gastos_envio;
    let total = orden.sub_total + orden.gastos_envio + descuento_monto.0 as i32;

    y_actual += 8.0;
    pdf.set_paragraph(
        &String::from("Subtotal"),
        16.0,
        y_actual + 2.0,
        70.0,
        -1,
        true,
    );
    let precio_subtotal = format_clp(orden.sub_total);
    y_actual = pdf.set_paragraph(&precio_subtotal, 16.0, y_actual + 2.0, 70.0, 1, false);

    if descuento_oferta > 0 {
        pdf.set_paragraph(
            &String::from("Descuento Oferta"),
            16.0,
            y_actual + 1.0,
            70.0,
            -1,
            true,
        );
        let precio_descuento_oferta = format_clp(descuento_oferta);
        y_actual = pdf.set_paragraph(
            &precio_descuento_oferta,
            16.0,
            y_actual + 1.0,
            70.0,
            1,
            false,
        );
    }
    if descuento_monto.0 > 0.0 && !descuento_monto.1 {
        pdf.set_paragraph(&descuento_monto.2, 16.0, y_actual + 1.0, 70.0, -1, true);
        let precio_descuento_monto = format_clp(descuento_monto.0 as i32);
        y_actual = pdf.set_paragraph(
            &precio_descuento_monto,
            16.0,
            y_actual + 1.0,
            70.0,
            1,
            false,
        );
    }
    if gastos_envio > 0 {
        pdf.set_paragraph(
            &String::from("Despacho"),
            16.0,
            y_actual + 1.0,
            70.0,
            -1,
            true,
        );
        let precio_gastos_envio = format_clp(gastos_envio);
        y_actual = pdf.set_paragraph(&precio_gastos_envio, 16.0, y_actual + 1.0, 70.0, 1, false);
    }
    if descuento_monto.0 > 0.0 && descuento_monto.1 {
        pdf.set_paragraph(&descuento_monto.2, 16.0, y_actual + 1.0, 70.0, -1, true);
        let precio_descuento_monto = format_clp(descuento_monto.0 as i32);
        y_actual = pdf.set_paragraph(
            &precio_descuento_monto,
            16.0,
            y_actual + 1.0,
            70.0,
            1,
            false,
        );
    }
    pdf.set_paragraph(&String::from("Total"), 16.0, y_actual + 1.0, 70.0, -1, true);
    let precio_total = format_clp(total);
    y_actual = pdf.set_paragraph(&precio_total, 16.0, y_actual + 1.0, 70.0, 1, false);
    // disclaimer
    let disclaimer = String::from("* Total no incluye propina ni cuota de servicio.");
    y_actual = pdf.set_paragraph(&disclaimer, 9.0, y_actual + 1.0, 70.0, -1, true);

    // medio de pago
    let medio_pago = &orden.pago.medio_pago.nombre;
    y_actual = pdf.set_paragraph(medio_pago, 16.0, y_actual + 5.0, 70.0, 1, false);

    let power_agil = String::from("powered by Agil");
    pdf.set_paragraph(&power_agil, 12.0, y_actual + 2.0, 80.0, 0, true);

    if is_copy {
        let reimpreso = String::from("*REIMPRESO*");
        // Agregamos tu texto o cualquier otro contenido necesario:
        pdf.set_paragraph(&reimpreso, 16.0, 5.0, 70.0, -1, false);
    }

    pdf.init_draw();
    pdf.drow_all_obj();
    pdf.save()
}
