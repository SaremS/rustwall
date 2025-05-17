pub struct PaywalledElement {
    access_unregistered: PaywallResponse,
    access_registered: PaywallResponse,
    access_paid: PaywallResponse 
}

pub enum PaywallResponse {
    HtmlTemplate(),
    HtmlRedirect()
}
