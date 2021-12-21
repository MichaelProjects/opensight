#import "OpensightCorePlugin.h"
#if __has_include(<opensight_core/opensight_core-Swift.h>)
#import <opensight_core/opensight_core-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "opensight_core-Swift.h"
#endif

@implementation OpensightCorePlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftOpensightCorePlugin registerWithRegistrar:registrar];
}
@end
