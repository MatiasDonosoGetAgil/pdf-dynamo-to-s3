## Markdown

```http
POST http://localhost:9000
Content-Type: application/json
{
    "texto": "client_credentials"
}
```

```http
POST http://localhost:9000
Content-Type: application/json
{
    "order": {
        "IdPedido": 277438,
        "IdSucursal": 769,
        "IdPublico": "53616c7465645f5f648e1c98e8999d2d482d121acc335712ffefbb6f560bda21",
        "VersionPOS": "2",
        "Codigo": "P34438",
        "Correlativo": 0,
        "Plataforma": {
            "Codigo": "AGIL",
            "Logo": "https://barbecue.getagil.com/Logos/Logo_Agil_Horizontal.png",
            "Nombre": "Agil"
        },
        "TipoPedido": {
            "Codigo": "PEDNORMAL",
            "Nombre": "Pedido normal",
            "Prefijo": "P"
        },
        "SubTotal": 1000,
        "GastosEnvio": 1501,
        "DsctoCuponSubtotal": 0,
        "DsctoCuponGastoEnvio": 0,
        "DsctoPuntos": 0,
        "CuotaServicioAgil": 0,
        "Propina": 0,
        "PropinaAgil": 0,
        "TotalOC": 2501,
        "Comentario": "PEDIDO DE PRUEBA AGIL",
        "TieneHijos": false,
        "IdPedidoBase": null,
        "IdCarro": 1647257,
        "Estado": {
            "Id": 3,
            "Nombre": "Rechazado",
            "Codigo": "REFUSED"
        },
        "ComentarioFlash": null,
        "EstadoDelivery": {
            "Id": 1,
            "Nombre": "Pendiente",
            "Codigo": "PENDING_DELIVERY"
        },
        "isDelivery": true,
        "isScheduled": true,
        "isOwnDelivery": false,
        "TipoFechaEntrega": {
            "Id": 3,
            "Tipo": "Programada"
        },
        "TipoEntrega": {
            "Id": 1,
            "Tipo": "Delivery"
        },
        "Cupones": [],
        "Pago": {
            "IdPublico": "53616c7465645f5f7b9389836a477a4a0dfd264ccded1f9439cd29d8900f155a",
            "Tarjeta": null,
            "IdPago": 267551,
            "MedioPago": {
                "Id": 14,
                "Nombre": "Efectivo",
                "Logo": "https://agil.s3.us-east-2.amazonaws.com/Logos/efectivo.png"
            },
            "TipoPago": {
                "Id": 4,
                "Tipo": "No especificado"
            },
            "EstadoPago": {
                "Id": 1,
                "Estado": "Activo"
            },
            "EstadoReembolso": "ERROR_REFUND",
            "FechaReembolso": null,
            "MontoReembolso": null
        },
        "Sucursal": {
            "IdSucursal": 769,
            "Nombre": "Completon Creta, Pte Alto",
            "Email": "beatriz.cerda1@gmail.com",
            "Telefono": "972160583",
            "Direccion": "Almirante Araya 1809, Santiago, Puente Alto, Región Metropolitana, Chile",
            "Lat": "-33.58070970",
            "Lng": "-70.60326620",
            "Comuna": "Puente Alto",
            "Ciudad": "Santiago",
            "Tz": "America/Santiago"
        },
        "Comercio": {
            "IdComercio": 489,
            "Nombre": "Completon",
            "CDN": "AGIL_SOFT",
            "Moneda": "CLP",
            "Iva": "19.00",
            "Logo": "https://agil.s3.us-east-2.amazonaws.com/comercio-prod/489/diseno/20241004131120_Logo.png?1728047480617"
        },
        "PickUp": {
            "Lat": "-33.58070970",
            "Lng": "-70.60326620",
            "Direccion": "Almirante Araya 1809, Santiago, Puente Alto, Región Metropolitana, Chile",
            "Ciudad": "Santiago",
            "Nombre": "Completon Creta, Pte Alto",
            "Evidencia": null
        },
        "DropOff": {
            "Lat": "-33.58680270",
            "Lng": "-70.60132640",
            "Direccion": "C. Creta 2869, 8150215 Puente Alto, Región Metropolitana, Chile",
            "Ciudad": "Santiago",
            "Distancia": 765,
            "Nombre": "soporte",
            "TipoEntrega": "Encontrarse en la puerta",
            "Evidencia": null
        },
        "Courier": {
            "IdCourier": null,
            "Plataforma": null,
            "Logo": null,
            "IdExterno": null,
            "Llave": 91309,
            "IdRider": null,
            "Nombre": null,
            "Telefono": null,
            "Imagen": "",
            "Lat": "0",
            "Lng": "0",
            "Distancia": null,
            "Bitacora": []
        },
        "Cliente": {
            "IdUsuario": 58803,
            "Nombre": "soporte",
            "Apellido": "agil ",
            "Email": "soporte@getagil.com",
            "Telefono": "961168802",
            "PrefijoTelefonico": 56,
            "Direccion": "C. Creta 2869, 8150215 Puente Alto, Región Metropolitana, Chile",
            "Lat": "-33.58680270",
            "Lng": "-70.60132640",
            "Nro": null
        },
        "Fechas": {
            "Tz": "America/Santiago",
            "FechaCreacion": "2024-11-21T19:34:13.000Z",
            "FechaPago": "2024-11-21T19:34:13.000Z",
            "FechaAceptacionEstimada": "2024-11-22T16:47:00.000Z",
            "FechaAceptacionReal": null,
            "FechaSalidaCocinaEstimada": "2024-11-22T16:56:00.000Z",
            "FechaSalidaCocinaReal": null,
            "FechaEntregaMin": "2024-11-22T16:58:00.000Z",
            "FechaEntregaMax": "2024-11-22T17:00:00.000Z",
            "FechaEntregaReal": null,
            "FechaSolicitudCourierEstimada": "2024-11-22T16:47:00.000Z",
            "FechaSolicitudCourierReal": null,
            "FechaLlegadaCourierEstimada": null,
            "FechaLlegadaCourierReal": null,
            "FechaReachazoMaximoSinCajero": null,
            "FechaRechazoReal": null,
            "FechaModificacion": null,
            "FechaPrimeraEvidencia": null,
            "FechaSegundaEvidencia": null,
            "TiempoPreparacion": "05:00",
            "TiempoCocina": "1-10",
            "TiempoDelivery": "0-2"
        },
        "Items": [
            {
                "IdProducto": 32925,
                "IdProductoExterno": null,
                "SKU": "PRODD9F931F8667C",
                "Nombre": "Aritos de Cebolla",
                "Descripcion": "5 unidades",
                "Imagen": "https://agil.s3.us-east-2.amazonaws.com/public/comercio-prod/489/productos/32925-aritosdecebolla-main-1280w.jpg?1728013846182",
                "Categoria": [
                    {
                        "IdCategoria": 7748,
                        "Nombre": "EMPANADAS%2CNUGGETS Y ARITOS DE CEBOLLA"
                    }
                ],
                "Cantidad": 1,
                "Precio": "1000.000",
                "PrecioFinal": "1000.000",
                "Total": "1000.000",
                "TotalFinal": "1000.000",
                "Comentario": null,
                "EsAgrupacion": null,
                "NombreAgrupacion": null,
                "CorrelativoAgrupacion": 0,
                "opciones": [
                    {
                        "Modificador": "MOD abc123",
                        "Cantidad": 999,
                        "Opcion": "abc123"
                    }
                ]
            }
        ],
        "Urls": [
            {
                "Tipo": "STATUS_AGIL",
                "Url": "https://www.completoncreta.cl/status?id=53616c7465645f5f648e1c98e8999d2d482d121acc335712ffefbb6f560bda21"
            },
            {
                "Tipo": "TICKET_THERMAL",
                "Url": "https://barbecue.getagil.com/comercio-prod/489/oc/tickets/POS/Ticket-P34438-277438-20241121193427.pdf"
            },
            {
                "Tipo": "TICKET_PUNTO",
                "Url": "https://barbecue.getagil.com/comercio-prod/489/oc/tickets/KITCHEN/Ticket-P34438-277438-20241121193427.txt"
            },
            {
                "Tipo": "TICKET_THERMAL_COPIA",
                "Url": "https://barbecue.getagil.com/comercio-prod/489/oc/tickets/POS/Ticket-P34438-277438-20241121193427.pdf"
            },
            {
                "Tipo": "TICKET_PUNTO_COPIA",
                "Url": "https://barbecue.getagil.com/comercio-prod/489/oc/tickets/KITCHEN/Ticket-P34438-277438-20241121193427.txt"
            }
        ],
        "Proveedores": [],
        "AlternativaDelivery": [
            {
                "IdCourier": -2,
                "Nombre": "AGIL",
                "Total": 1501,
                "CuotaServicioCourier": {},
                "TipoTarifa": "SUCURSAL",
                "Distancia": 765,
                "DistanciaTarifaDesde": 0,
                "DistanciaTarifaHasta": 1,
                "PrecioBaseClienteNeto": 1950,
                "PrecioBaseRiderNeto": 1300,
                "PrecioExtraCliente": 50,
                "PrecioExtraRider": 20,
                "CuotaServicioAgilNeto": 100,
                "CuotaServicioAgilIva": 19,
                "CuotaServicioAgilBruto": 119
            },
            {
                "IdCourier": 2,
                "Nombre": "UBER DIRECT",
                "Total": 2013.01,
                "CuotaServicioCourier": {
                    "CuotaServicioNeto": 279,
                    "CuotaServicioIva": 53.01,
                    "CuotaServicioBruto": 332.01,
                    "GastoEnvioNeto": 1800,
                    "GastoEnvioIva": 342,
                    "GastoEnvioBruto": 2142,
                    "Desde": 0,
                    "Hasta": 1
                }
            },
            {
                "IdCourier": -1,
                "Nombre": "AGIL",
                "Total": 2320.5,
                "CuotaServicioCourier": {
                    "CuotaServicioNeto": 0,
                    "CuotaServicioIva": 0,
                    "CuotaServicioBruto": 0,
                    "GastoEnvioNeto": 0,
                    "GastoEnvioIva": 0,
                    "GastoEnvioBruto": 0
                }
            }
        ]
    }
}
```