#[test]
fn compute_pass_dispatch_global_invocation_index_overflow() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::NOOP,
        backend_options: wgpu::BackendOptions {
            noop: wgpu::NoopBackendOptions::enabled(),
            ..Default::default()
        },
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    });

    let adapter =
        pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
            .expect("should be able to request adapter");
    let (device, _queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        required_features: wgpu::Features::LINEAR_INDEXING,
        ..Default::default()
    }))
    .expect("should be able to get device");

    // Shader using global_invocation_index with linear_indexing
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("compute shader with global_invocation_index that will overflow"),
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
            r#"
enable linear_indexing;

@compute @workgroup_size(4, 8, 8)
fn main(@builtin(global_invocation_index) gindex : u32) {

}
            "#,
        )),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("layout"),
        bind_group_layouts: &[],
        immediate_size: 0,
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("compute pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: Some("main"),
        compilation_options: Default::default(),
        cache: None,
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("encoder"),
    });

    let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
        label: Some("compute pass"),
        timestamp_writes: None,
    });

    compute_pass.set_pipeline(&pipeline);

    let large_dispatch = u16::MAX as u32;
    compute_pass.dispatch_workgroups(large_dispatch, large_dispatch, large_dispatch);

    drop(compute_pass);
    encoder.finish();
}
