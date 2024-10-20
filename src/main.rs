extern crate qtip_rs;
use qtip_rs::*;

fn main() {
    let max_elements = 10;
    let element_size = std::mem::size_of::<usize>();
    let mut data: Box<[usize]> = vec![0; 10].into_boxed_slice();

    let mut context = qtipContext_t {
        maxItems: 0,
        qty: 0,
        start: std::ptr::null_mut(),
        front: 0,
        rear: 0,
        itemSize: 0,
        locked: false,
        processed: 0,
        total: 0,
    };

    unsafe {
        qtip_init(
            &mut context,
            Box::into_raw(data) as *mut std::ffi::c_void,
            max_elements,
            element_size,
        );

        let mut element: usize = 0;
        let offset = 3;
        for i in (offset)..(max_elements + offset) {
            let status = qtip_put(&mut context, &i as *const _ as *mut std::ffi::c_void);
            assert_eq!(status, qtipStatus_t_QTIP_STATUS_OK);
        }
        qtip_get_front(&mut context, &element as *const _ as *mut std::ffi::c_void);
        println!("Element at front: {}", element);

        qtip_get_rear(&mut context, &element as *const _ as *mut std::ffi::c_void);
        println!("Element at rear: {}", element);

        let status = qtip_put(&mut context, &element as *const _ as *mut std::ffi::c_void);
        assert_eq!(
            status, qtipStatus_t_QTIP_STATUS_OK,
            "Expected QTIP_STATUS_OK, got {:?}",
            status
        );
    }
}
