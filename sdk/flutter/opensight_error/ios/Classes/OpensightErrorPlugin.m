#import "OpensightErrorPlugin.h"
#if __has_include(<opensight_error/opensight_error-Swift.h>)
#import <opensight_error/opensight_error-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "opensight_error-Swift.h"
#endif

@implementation OpensightErrorPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftOpensightErrorPlugin registerWithRegistrar:registrar];
}
@end
