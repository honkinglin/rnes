# M4 Module Testing

## Overview

M4模块测试涵盖了Common Mappers和Save System的功能验证。测试包括：

1. **Mapper功能测试**：验证MMC1、UxROM、CNROM、AOROM等常见mapper的正确实现
2. **Save System测试**：验证电池备份和存档状态功能
3. **集成测试**：验证mapper和save system的协同工作

## Test Structure

### Test Files

- `crates/test-suite/src/m4_integration_tests.rs` - 主要测试文件
- `crates/test-suite/src/m4_test_runner.rs` - M4专用测试运行器

### Test Categories

#### 1. Basic Functionality Tests
- `test_m4_basic_functionality()` - 基本功能验证
- `test_mapper_creation()` - Mapper创建测试

#### 2. Save System Tests
- `test_save_system_basic()` - 基础save system功能
- `test_save_state_functionality()` - 存档状态功能
- `test_mmc1_mapper_with_save_system()` - MMC1与save system集成

#### 3. Mapper Tests (Require Test ROMs)
- `test_mmc1_mapper()` - MMC1 mapper测试
- `test_uxrom_mapper()` - UxROM mapper测试
- `test_cnrom_mapper()` - CNROM mapper测试
- `test_aorom_mapper()` - AOROM mapper测试

#### 4. Integration Tests
- `test_m4_integration()` - 完整集成测试
- `test_m4_with_test_runner()` - 使用专用测试运行器

## Running Tests

### Basic Tests (No ROMs Required)

```bash
# Run all M4 basic tests
cargo test -p rnes-test-suite --test m4_integration_tests

# Run specific test
cargo test -p rnes-test-suite --test m4_integration_tests test_save_system_basic
```

### Full Tests (Require Test ROMs)

```bash
# Download M4 test ROMs
./scripts/download_m4_test_roms.sh

# Run all M4 tests including ROM-based tests
cargo test -p rnes-test-suite --test m4_integration_tests -- --ignored
```

## Test Runner

### M4TestRunner

专门为M4模块设计的测试运行器，提供以下功能：

```rust
let mut runner = M4TestRunner::new()
    .with_max_cycles(1000000)
    .with_save_system(true)
    .with_battery_backup(true)
    .with_save_state_slots(vec![1, 2, 3]);
```

#### Configuration Options

- `max_cycles`: 最大执行周期数
- `save_system`: 是否启用save system测试
- `battery_backup`: 是否启用电池备份测试
- `save_state_slots`: 要测试的存档槽位

#### Test Results

```rust
pub enum M4TestResult {
    Completed {
        status: u8,
        cycles: u32,
        save_states_created: usize,
        battery_backups_saved: usize,
    },
    Timeout { cycles: u32 },
    InfiniteLoop { cycles: u32, pc: u16 },
    Error { error: String, cycles: u32 },
}
```

## Test Coverage

### Mapper Testing

#### Supported Mappers
- ✅ **NROM (0)** - 基础mapper
- ✅ **MMC1 (1)** - 支持电池备份
- ✅ **UxROM (2)** - 简单bank switching
- ✅ **CNROM (3)** - CHR ROM bank switching
- ✅ **AOROM (7)** - 简单PRG bank switching

#### Test Features
- Mapper创建和初始化
- Bank switching功能
- 内存映射验证
- 电池备份支持检测

### Save System Testing

#### Battery Backup
- 数据保存和加载
- 文件存在性检查
- 数据完整性验证
- 错误处理

#### Save States
- 多槽位支持
- 状态保存和恢复
- CPU状态验证
- PPU状态验证
- 内存状态验证

## Test ROMs

### Required Test ROMs

M4测试需要以下测试ROM：

```
tests/roms/nes-test-roms/
├── mmc1_tests/
│   ├── mmc1_test.nes
│   └── mmc1_basic.nes
├── uxrom_tests/
│   ├── uxrom_test.nes
│   └── uxrom_basic.nes
├── cnrom_tests/
│   ├── cnrom_test.nes
│   └── cnrom_basic.nes
└── aorom_tests/
    ├── aorom_test.nes
    └── aorom_basic.nes
```

### Download Script

```bash
./scripts/download_m4_test_roms.sh
```

## Test Results

### Expected Output

```
🧪 Testing M4 Integration (Mappers + Save System)
Testing Mapper 1 integration...
✅ Mapper 1 integration test passed
Testing Mapper 2 integration...
✅ Mapper 2 integration test passed
Testing Mapper 3 integration...
✅ Mapper 3 integration test passed
Testing Mapper 7 integration...
✅ Mapper 7 integration test passed
✅ M4 integration test completed

🧪 Testing Save System Basic Functionality
✅ Save system creation test passed
✅ Battery backup save test passed
✅ Battery backup load test passed
✅ Battery backup existence check passed
✅ Battery backup cleanup passed
```

### Success Criteria

- 所有mapper创建成功
- Save system功能正常
- 电池备份保存和加载正确
- 存档状态创建和恢复正确
- 集成测试通过

## Troubleshooting

### Common Issues

1. **Memory Access Errors**
   - 确保测试ROM包含正确的PRG RAM
   - 检查mapper的电池备份实现

2. **Save State Failures**
   - 验证ROM名称设置正确
   - 检查save system目录权限

3. **Test ROM Not Found**
   - 运行下载脚本：`./scripts/download_m4_test_roms.sh`
   - 检查ROM文件路径

### Debug Mode

```bash
# Run with debug output
RUST_LOG=debug cargo test -p rnes-test-suite --test m4_integration_tests

# Run specific test with backtrace
RUST_BACKTRACE=1 cargo test -p rnes-test-suite --test m4_integration_tests test_save_system_basic
```

## Future Enhancements

### Planned Tests

1. **Performance Tests**
   - Mapper切换性能
   - Save system I/O性能

2. **Stress Tests**
   - 大量存档状态创建
   - 长时间运行稳定性

3. **Compatibility Tests**
   - 不同ROM格式支持
   - 向后兼容性验证

### Test Infrastructure

1. **Automated ROM Generation**
   - 动态生成测试ROM
   - 自定义mapper配置

2. **Continuous Integration**
   - GitHub Actions集成
   - 自动化测试报告

## Conclusion

M4模块测试提供了全面的功能验证，确保：

- 所有支持的mapper正确实现
- Save system功能完整可靠
- 模块间集成正常工作
- 错误处理机制有效

通过运行这些测试，可以确保M4模块的质量和稳定性。
