
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


    return Location
}
