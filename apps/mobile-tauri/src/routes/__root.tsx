import { Link, Outlet, RootRoute } from '@tanstack/react-router'
import { Suspense, lazy } from 'react'

const TanStackRouterDevtools =
    import.meta.env.PROD
        ? () => null // Render nothing in production
        : lazy(() =>
            // Lazy load in development
            import('@tanstack/router-devtools').then((res) => ({
                default: res.TanStackRouterDevtools,
            })),
        )

export const Route = new RootRoute({
    component: RootComponent,
})

function RootComponent() {
    return (
        <>
            <div className="p-2 flex gap-2">
                <Link to="/" className="[&.active]:font-bold">
                    Home
                </Link>{' '}
                <Link to="/about" className="[&.active]:font-bold">
                    About
                </Link>
            </div>
            <hr />

            <Outlet />
            <Suspense>
                <TanStackRouterDevtools position="bottom-right" />
            </Suspense>
        </>
    )
}
