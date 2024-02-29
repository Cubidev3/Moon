

#[macro_export]
macro_rules! vector_base {
     ($vector_name:ident<$component_type: ty> {$($component:ident),*}) => {
          #[derive(Copy, Clone, PartialEq, Debug)]
          pub struct $vector_name {
               $( pub $component: $component_type, )*
          }

          impl Add for $vector_name {
               type Output = $vector_name;

               fn add(self, rhs: Self) -> Self::Output {
                    $vector_name { $($component: self.$component + rhs.$component, )* }
               }
          }

          impl AddAssign for $vector_name {
               fn add_assign(&mut self, rhs: Self) {
                    $(self.$component += rhs.$component;)*
               }
          }

          impl Sub for $vector_name {
               type Output = $vector_name;

               fn sub(self, rhs: Self) -> Self::Output {
                    $vector_name { $($component: self.$component - rhs.$component, )* }
               }
          }

          impl SubAssign for $vector_name {
               fn sub_assign(&mut self, rhs: Self) {
                    $(self.$component -= rhs.$component;)*
               }
          }

          impl Mul<$component_type> for $vector_name {
               type Output = $vector_name;

               fn mul(self, rhs: $component_type) -> Self::Output {
                    $vector_name { $($component: self.$component * rhs, )* }
               }
          }

          impl Mul<$vector_name> for $component_type {
               type Output = $vector_name;

               fn mul(self, rhs: $vector_name) -> Self::Output {
                    $vector_name { $($component: self * rhs.$component, )* }
               }
          }

          impl MulAssign<$component_type> for $vector_name {
               fn mul_assign(&mut self, rhs: $component_type) {
                    $(self.$component *= rhs;)*
               }
          }

          impl Div<$component_type> for $vector_name {
               type Output = $vector_name;

               fn div(self, rhs: $component_type) -> Self::Output {
                    $vector_name { $($component: self.$component / rhs, )* }
               }
          }

          impl DivAssign<$component_type> for $vector_name {
               fn div_assign(&mut self, rhs: $component_type) {
                    $(self.$component /= rhs;)*
               }
          }

          impl Neg for $vector_name {
               type Output = $vector_name;

               fn neg(self) -> Self::Output {
                    $vector_name { $($component: -self.$component, )* }
               }
          }
     };
}